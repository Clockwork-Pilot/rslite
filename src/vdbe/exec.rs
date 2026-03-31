//! VDBE register types and the `step()` execution loop.

use super::{Opcode, P4, Program};
use super::storage::{SQLITE_ERROR, SQLITE_CORRUPT, SQLITE_ROW, SQLITE_DONE};
use snafu::Snafu;
use std::collections::HashMap;

pub use super::storage::{MemStorage, MemCursor, StorageBackend, TableCursor};



#[derive(Debug, Snafu)]
pub enum ExecError {
    #[snafu(display("Column index {} out of bounds for row with {} columns", index, len))]
    ColumnIndexOutOfBounds { index: usize, len: usize },

    #[snafu(display("Table '{}' not found", table))]
    TableNotFound { table: String },

    #[snafu(display("Cursor slot {} not open", slot))]
    CursorNotOpen { slot: usize },

    #[snafu(display("Invalid rowid: {}", rowid))]
    InvalidRowid { rowid: i64 },

    #[snafu(display("Empty record in insert"))]
    EmptyRecord,

    #[snafu(display("Cursor {} has no current row", slot))]
    NoCursorRow { slot: usize },

    #[snafu(display("Register index {} out of bounds", index))]
    RegisterOutOfBounds { index: usize },

    #[snafu(display("Data corruption: {}", reason))]
    DataCorruption { reason: String },

    #[snafu(display("Invalid instruction at pc={}", pc))]
    InvalidInstruction { pc: usize },

    #[snafu(display("Generic error: {}", reason))]
    Generic { reason: String },
}

impl ExecError {
    pub fn sqlite_code(&self) -> i32 {
        match self {
            Self::DataCorruption { .. } => SQLITE_CORRUPT,
            _ => SQLITE_ERROR,
        }
    }

    pub fn log_error(&self) {
        eprintln!(
            "[rust-sqlite] EXEC ERROR ({}): {}",
            self.sqlite_code(),
            self
        );
    }
}

impl From<i32> for ExecError {
    fn from(code: i32) -> Self {
        ExecError::Generic {
            reason: format!("SQLite error code {}", code),
        }
    }
}

// ── OrdRegister — wrapper for BTreeMap keys with total ordering ────────────────

/// Wrapper around Register that implements Ord for use in BTreeMap.
/// Provides a total order where Null < numeric/text < blob.
#[derive(Debug, Clone)]
pub struct OrdRegister(pub Register);

impl PartialEq for OrdRegister {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for OrdRegister {}

impl Ord for OrdRegister {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        use Register::*;
        match (&self.0, &other.0) {
            (Null, Null) => Ordering::Equal,
            (Null, _) => Ordering::Less,
            (_, Null) => Ordering::Greater,
            (a, b) => a.cmp(b).unwrap_or(Ordering::Equal),
        }
    }
}

impl PartialOrd for OrdRegister {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// ── Register ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl Register {
    pub fn to_i64(&self) -> i64 {
        match self {
            Register::Integer(v) => *v,
            Register::Real(v) => *v as i64,
            Register::Text(s) => s.parse().unwrap_or(0),
            Register::Null | Register::Blob(_) => 0,
        }
    }
    pub fn to_f64(&self) -> f64 {
        match self {
            Register::Integer(v) => *v as f64,
            Register::Real(v) => *v,
            Register::Text(s) => s.parse().unwrap_or(0.0),
            Register::Null | Register::Blob(_) => 0.0,
        }
    }
    pub fn is_null(&self) -> bool {
        matches!(self, Register::Null)
    }
    pub fn is_truthy(&self) -> bool {
        match self {
            Register::Null => false,
            Register::Integer(v) => *v != 0,
            Register::Real(v) => *v != 0.0,
            Register::Text(s) => !s.is_empty() && s != "0",
            Register::Blob(b) => !b.is_empty(),
        }
    }
    pub fn to_text(&self) -> String {
        match self {
            Register::Null => String::new(),
            Register::Integer(v) => v.to_string(),
            Register::Real(v) => v.to_string(),
            Register::Text(s) => s.clone(),
            Register::Blob(_) => String::new(),
        }
    }

    pub fn cmp(&self, other: &Register) -> Option<std::cmp::Ordering> {
        use Register::*;
        match (self, other) {
            (Null, _) | (_, Null) => None,
            (Integer(a), Integer(b)) => Some(a.cmp(b)),
            (Real(a), Real(b)) => a.partial_cmp(b),
            (Integer(a), Real(b)) => (*a as f64).partial_cmp(b),
            (Real(a), Integer(b)) => a.partial_cmp(&(*b as f64)),
            (Text(a), Text(b)) => Some(a.cmp(b)),
            (Blob(a), Blob(b)) => Some(a.cmp(b)),
            (Integer(_) | Real(_), _) => Some(std::cmp::Ordering::Less),
            (_, Integer(_) | Real(_)) => Some(std::cmp::Ordering::Greater),
            (Text(_), Blob(_)) => Some(std::cmp::Ordering::Less),
            (Blob(_), Text(_)) => Some(std::cmp::Ordering::Greater),
        }
    }

    /// Alias for cmp (used in sorting)
    pub fn compare(&self, other: &Register) -> Option<std::cmp::Ordering> {
        self.cmp(other)
    }

    pub fn sqlite_type(&self) -> i32 {
        match self {
            Register::Integer(_) => 1,
            Register::Real(_) => 2,
            Register::Text(_) => 3,
            Register::Blob(_) => 4,
            Register::Null => 5,
        }
    }
}

// ── In-memory table ───────────────────────────────────────────────────────────

#[derive(Debug, Default, Clone)]
pub struct MemTable {
    pub rows: std::collections::BTreeMap<i64, Vec<Register>>,
    pub next_rowid: i64,
}

impl MemTable {
    pub fn new() -> Self {
        MemTable {
            rows: std::collections::BTreeMap::new(),
            next_rowid: 1,
        }
    }
    pub fn alloc_rowid(&mut self) -> i64 {
        let id = self.next_rowid;
        self.next_rowid += 1;
        id
    }
}

// ── Aggregate accumulator ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum AggState {
    Count(i64),
    Sum(f64, bool), // (accumulator, seen_non_null)
    Avg(f64, i64),  // (sum, count)
    MinVal(Option<Register>),
    MaxVal(Option<Register>),
    GroupConcat(Vec<String>, Option<String>), // (values, separator)
    Total(f64),                               // always returns float; 0.0 if no rows
    Unset,
}

// ── Sort buffer entry ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SortEntry {
    pub result_cols: Vec<Register>,
    pub sort_keys: Vec<Register>,
    /// Compound-select tag: 0 = left, 1 = UNION, 2 = UNION ALL, 3 = INTERSECT, 4 = EXCEPT
    pub tag: i32,
}


// ── VM state ──────────────────────────────────────────────────────────────────

/// The VDBE execution state, generic over the storage backend.
///
/// Use `Vm<MemStorage>` for in-memory tables (the default).
/// Use `Vm<BtreeStorage>` to execute against the real pager/B-tree layer.
pub struct Vm<S: StorageBackend> {
    pub pc: usize,
    pub regs: Vec<Register>,
    pub bind_params: Vec<Register>,
    pub result_row: Vec<Register>,
    pub halted: bool,

    /// Pluggable storage: factory for cursors and table-level operations.
    pub backend: S,
    /// Open cursor slots; index = p1 of Open*.
    pub cursors: Vec<Option<S::Cursor>>,
    /// Index-scan state per cursor slot: (rowids, current_position).
    /// Set by SeekEq; consumed by IdxNext.
    pub index_states: HashMap<usize, (Vec<i64>, usize)>,
    /// Pending typed record from MakeRecord, consumed by Insert.
    pub pending_record: Option<Vec<Register>>,
    /// Change count for current statement.
    pub changes_in_stmt: i64,
    /// Last inserted rowid for current statement.
    pub last_insert_rowid_in_stmt: i64,
    /// Aggregate state slots keyed by register index.
    pub agg_states: HashMap<i32, AggState>,
    /// Sort buffer for ORDER BY / compound selects / GROUP BY.
    pub sort_buffer: Vec<SortEntry>,
    /// Materialised output rows ready to emit one by one.
    pub result_rows: Vec<Vec<Register>>,
    /// Index into result_rows for emitting.
    pub result_index: usize,
    /// DISTINCT seen-set: serialised row keys.
    pub distinct_seen: std::collections::HashSet<String>,
}

impl<S: StorageBackend> Vm<S> {
    pub fn new(n_regs: usize, n_vars: u32, backend: S) -> Self {
        Vm {
            pc: 0,
            regs: vec![Register::Null; n_regs.max(256)],
            bind_params: vec![Register::Null; n_vars as usize],
            result_row: Vec::new(),
            halted: false,
            backend,
            cursors: Vec::new(),
            index_states: HashMap::new(),
            pending_record: None,
            changes_in_stmt: 0,
            last_insert_rowid_in_stmt: 0,
            agg_states: HashMap::new(),
            sort_buffer: Vec::new(),
            result_rows: Vec::new(),
            result_index: 0,
            distinct_seen: std::collections::HashSet::new(),
        }
    }

    pub fn set_bind_param(&mut self, idx: usize, val: Register) {
        if idx < self.bind_params.len() {
            self.bind_params[idx] = val;
        }
    }

    fn ensure_cursor_slot(&mut self, slot: usize) {
        while self.cursors.len() <= slot {
            self.cursors.push(None);
        }
    }

    /// Replace the cursor in `slot` with a freshly opened one.
    fn open_cursor_slot(&mut self, slot: usize, table_name: &str, writable: bool) -> Result<(), i32> {
        let cursor = self.backend.open_cursor(table_name, writable)?;
        self.ensure_cursor_slot(slot);
        self.cursors[slot] = Some(cursor);
        Ok(())
    }

    fn ensure_reg(&mut self, idx: usize) {
        if idx >= self.regs.len() {
            self.regs.resize(idx + 1, Register::Null);
        }
    }
}

// ── Arithmetic helpers ────────────────────────────────────────────────────────

fn arith(
    a: &Register,
    b: &Register,
    int_fn: fn(i64, i64) -> Option<i64>,
    flt_fn: fn(f64, f64) -> f64,
) -> Register {
    use Register::*;
    match (a, b) {
        (Null, _) | (_, Null) => Null,
        (Integer(a), Integer(b)) => match int_fn(*a, *b) {
            Some(v) => Integer(v),
            None => Real(*a as f64 + *b as f64),
        },
        _ => Real(flt_fn(a.to_f64(), b.to_f64())),
    }
}

// ── step() ────────────────────────────────────────────────────────────────────

pub fn step<S: StorageBackend>(prog: &Program, vm: &mut Vm<S>) -> Result<i32, ExecError> {

    if vm.result_index < vm.result_rows.len() {
        vm.result_row = vm.result_rows[vm.result_index].clone();
        vm.result_index += 1;
        return Ok(SQLITE_ROW);
    }
    if vm.halted {
        return Ok(SQLITE_DONE);
    }

    loop {
        let instr = prog
            .instrs
            .get(vm.pc)
            .ok_or_else(|| ExecError::InvalidInstruction { pc: vm.pc })?;
        vm.pc += 1;

        let (p1, p2, p3) = (instr.p1, instr.p2, instr.p3);

        // Only opcodes where p1/p2/p3 are register indexes need to pre-allocate
        let need = match &instr.op {
            // For value-loading opcodes, only p2/p3 are registers
            Opcode::Integer | Opcode::Int64 | Opcode::Real | Opcode::String8 | Opcode::Blob => {
                (p2.max(p3) as usize + 1).max(1)
            }
            // For most other opcodes, all three are register indexes
            _ => (p1.max(p2).max(p3) as usize + 1).max(1),
        };
        if need > vm.regs.len() {
            vm.regs.resize(need, Register::Null);
        }

        macro_rules! reg {
            ($n:expr) => {
                vm.regs[$n as usize]
            };
        }

        match &instr.op {
            // ── Control ──────────────────────────────────────────────────────
            Opcode::Init => {
                if p2 != 0 {
                    vm.pc = p2 as usize;
                }
                // p2 == 0: do nothing, vm.pc already advanced past Init
            }
            Opcode::Goto => {
                vm.pc = p2 as usize;
            }
            Opcode::Noop => {}
            Opcode::Halt => {
                vm.halted = true;
                return if p1 == 0 {
                    Ok(SQLITE_DONE)
                } else {
                    Err(ExecError::Generic {
                        reason: format!("Halt with error code {}", p1),
                    })
                };
            }

            // ── Constant loads ────────────────────────────────────────────────
            Opcode::Null => {
                reg!(p2) = Register::Null;
            }
            Opcode::Integer => {
                reg!(p2) = Register::Integer(p1 as i64);
            }
            Opcode::Int64 => {
                if let P4::Int(v) = &instr.p4 {
                    reg!(p2) = Register::Integer(*v);
                }
            }
            Opcode::Real => {
                if let P4::Real(v) = &instr.p4 {
                    reg!(p2) = Register::Real(*v);
                }
            }
            Opcode::String8 => {
                if let P4::Text(s) = &instr.p4 {
                    reg!(p2) = Register::Text(s.clone());
                }
            }
            Opcode::Blob => {
                if let P4::Blob(b) = &instr.p4 {
                    reg!(p2) = Register::Blob(b.clone());
                }
            }

            // ── Register copy ─────────────────────────────────────────────────
            Opcode::Copy => {
                reg!(p2) = reg!(p1).clone();
            }

            // ── Arithmetic ────────────────────────────────────────────────────
            Opcode::Add => {
                let r = arith(&reg!(p1), &reg!(p2), i64::checked_add, |a, b| a + b);
                reg!(p3) = r;
            }
            Opcode::Subtract => {
                let r = arith(&reg!(p1), &reg!(p2), i64::checked_sub, |a, b| a - b);
                reg!(p3) = r;
            }
            Opcode::Multiply => {
                let r = arith(&reg!(p1), &reg!(p2), i64::checked_mul, |a, b| a * b);
                reg!(p3) = r;
            }
            Opcode::Divide => {
                let r = if reg!(p2).to_i64() == 0 || reg!(p2).is_null() {
                    Register::Null
                } else {
                    arith(&reg!(p1), &reg!(p2), i64::checked_div, |a, b| a / b)
                };
                reg!(p3) = r;
            }
            Opcode::Remainder => {
                let r = if reg!(p2).to_i64() == 0 || reg!(p2).is_null() {
                    Register::Null
                } else {
                    arith(&reg!(p1), &reg!(p2), i64::checked_rem, |a, b| a % b)
                };
                reg!(p3) = r;
            }

            // ── Bitwise ───────────────────────────────────────────────────────
            Opcode::BitAnd => {
                let v = reg!(p1).to_i64() & reg!(p2).to_i64();
                reg!(p3) = Register::Integer(v);
            }
            Opcode::BitOr => {
                let v = reg!(p1).to_i64() | reg!(p2).to_i64();
                reg!(p3) = Register::Integer(v);
            }
            Opcode::ShiftLeft => {
                let v = reg!(p1).to_i64() << (reg!(p2).to_i64() & 63);
                reg!(p3) = Register::Integer(v);
            }
            Opcode::ShiftRight => {
                let v = reg!(p1).to_i64() >> (reg!(p2).to_i64() & 63);
                reg!(p3) = Register::Integer(v);
            }
            Opcode::BitNot => {
                let v = !reg!(p1).to_i64();
                reg!(p2) = Register::Integer(v);
            }

            // ── String ────────────────────────────────────────────────────────
            Opcode::Concat => {
                let r = match (reg!(p1).clone(), reg!(p2).clone()) {
                    (Register::Null, _) | (_, Register::Null) => Register::Null,
                    (Register::Text(a), Register::Text(b)) => Register::Text(a + &b),
                    (a, b) => Register::Text(format!("{}{}", a.to_i64(), b.to_i64())),
                };
                reg!(p3) = r;
            }

            // ── Logical ───────────────────────────────────────────────────────
            Opcode::And => {
                let (a, b) = (reg!(p1).clone(), reg!(p2).clone());
                reg!(p3) = match (a.is_null(), a.is_truthy(), b.is_null(), b.is_truthy()) {
                    (_, false, _, _) | (_, _, _, false) => Register::Integer(0),
                    (true, _, _, _) | (_, _, true, _) => Register::Null,
                    _ => Register::Integer(1),
                };
            }
            Opcode::Or => {
                let (a, b) = (reg!(p1).clone(), reg!(p2).clone());
                reg!(p3) = match (a.is_null(), a.is_truthy(), b.is_null(), b.is_truthy()) {
                    (_, true, _, _) | (_, _, _, true) => Register::Integer(1),
                    (true, _, _, _) | (_, _, true, _) => Register::Null,
                    _ => Register::Integer(0),
                };
            }
            Opcode::Not => {
                reg!(p2) = match &reg!(p1) {
                    Register::Null => Register::Null,
                    r => Register::Integer(!r.is_truthy() as i64),
                };
            }

            // ── Comparisons ───────────────────────────────────────────────────
            Opcode::Eq | Opcode::Ne | Opcode::Lt | Opcode::Le | Opcode::Gt | Opcode::Ge => {
                let (lhs, rhs) = (reg!(p1).clone(), reg!(p3).clone());
                let null_eq = (instr.p5 & 0x80) != 0;
                let ord = match (lhs.is_null(), rhs.is_null()) {
                    (true, true) if null_eq => Some(std::cmp::Ordering::Equal),
                    (true, _) | (_, true) => None,
                    _ => lhs.cmp(&rhs),
                };
                let jump = ord
                    .map(|o| match &instr.op {
                        Opcode::Eq => o == std::cmp::Ordering::Equal,
                        Opcode::Ne => o != std::cmp::Ordering::Equal,
                        Opcode::Lt => o == std::cmp::Ordering::Less,
                        Opcode::Le => o != std::cmp::Ordering::Greater,
                        Opcode::Gt => o == std::cmp::Ordering::Greater,
                        Opcode::Ge => o != std::cmp::Ordering::Less,
                        _ => unreachable!(),
                    })
                    .unwrap_or(false);
                if jump {
                    vm.pc = p2 as usize;
                }
            }

            // ── NULL tests ────────────────────────────────────────────────────
            Opcode::IsNull => {
                if reg!(p1).is_null() {
                    vm.pc = p2 as usize;
                }
            }
            Opcode::NotNull => {
                if !reg!(p1).is_null() {
                    vm.pc = p2 as usize;
                }
            }

            // ── Conditional jumps ─────────────────────────────────────────────
            Opcode::If => {
                if reg!(p1).is_truthy() {
                    vm.pc = p2 as usize;
                }
            }
            Opcode::IfNot => {
                if !reg!(p1).is_truthy() {
                    vm.pc = p2 as usize;
                }
            }

            // ── Bind parameters ───────────────────────────────────────────────
            Opcode::Variable => {
                let idx = (p1 - 1) as usize;
                reg!(p2) = vm.bind_params.get(idx).cloned().unwrap_or(Register::Null);
            }

            // ── Output ────────────────────────────────────────────────────────
            Opcode::ResultRow => {
                let first = p1 as usize;
                let count = p2 as usize;
                if first + count > vm.regs.len() {
                    return Err(SQLITE_ERROR.into());
                }
                vm.result_row = vm.regs[first..first + count].to_vec();
                return Ok(SQLITE_ROW);
            }

            // ── Built-in functions ────────────────────────────────────────────
            Opcode::Function => {
                let argc = p1 as usize;
                let base = p2 as usize;
                let dst = p3 as usize;
                if base + argc > vm.regs.len() {
                    return Err(SQLITE_ERROR.into());
                }
                let args = vm.regs[base..base + argc].to_vec();
                vm.regs[dst] = match &instr.p4 {
                    P4::FuncName(name) => call_builtin(name, &args),
                    _ => Register::Null,
                };
            }

            // ── Transaction ───────────────────────────────────────────────────
            // p2=0 read-only, p2=1 write — we have no WAL so this is a no-op.
            Opcode::Transaction => {}

            // ── Cursor open ───────────────────────────────────────────────────
            // p1=slot, p2=root_page, p3=ncols, p4=Text(table_name)
            Opcode::OpenRead | Opcode::OpenWrite => {
                let slot = p1 as usize;
                let tname = match &instr.p4 {
                    P4::Text(s) => s.clone(),
                    _ => return Err(SQLITE_ERROR.into()),
                };
                let writable = matches!(&instr.op, Opcode::OpenWrite);
                vm.open_cursor_slot(slot, &tname, writable)
                    .map_err(|_| ExecError::TableNotFound { table: tname })?;
            }
            
            // ── Cursor close ─────────────────────────────────────────────────
            Opcode::Close => {
                let slot = p1 as usize;
                if slot < vm.cursors.len() {
                    vm.cursors[slot] = None;
                }
            }

            // ── Rewind ───────────────────────────────────────────────────────
            // p1=cursor, p2=jump_if_empty
            Opcode::Rewind => {
                let slot = p1 as usize;
                let empty = vm.cursors.get_mut(slot)
                    .and_then(|c| c.as_mut())
                    .ok_or(SQLITE_ERROR)?
                    .rewind()
                    .map_err(|_| SQLITE_ERROR)?;
                if empty {
                    vm.pc = p2 as usize;
                }
            }

            // ── Next ─────────────────────────────────────────────────────────
            // p1=cursor, p2=jump_back_if_more (loops back to scan body)
            Opcode::Next => {
                let slot = p1 as usize;
                let has_more = vm.cursors.get_mut(slot)
                    .and_then(|c| c.as_mut())
                    .ok_or(SQLITE_ERROR)?
                    .next()
                    .map_err(|_| SQLITE_ERROR)?;
                if has_more {
                    vm.pc = p2 as usize;
                    unsafe { crate::src::src::vdbe::sqlite3_search_count += 1; }
                }
            }

            // ── Prev ─────────────────────────────────────────────────────────
            // p1=cursor, p2=jump_back_if_more
            Opcode::Prev => {
                let slot = p1 as usize;
                let has_more = vm.cursors.get_mut(slot)
                    .and_then(|c| c.as_mut())
                    .ok_or(SQLITE_ERROR)?
                    .prev()
                    .map_err(|_| SQLITE_ERROR)?;
                if has_more {
                    vm.pc = p2 as usize;
                }
            }

            // ── Column ───────────────────────────────────────────────────────
            // p1=cursor, p2=col_index, p3=dst_reg
            Opcode::Column => {
                let slot = p1 as usize;
                let col = p2 as usize;
                let dst = p3 as usize;
                let val = vm.cursors.get_mut(slot)
                    .and_then(|c| c.as_mut())
                    .ok_or(SQLITE_ERROR)?
                    .column(col)
                    .map_err(|_| SQLITE_ERROR)?;
                if dst >= vm.regs.len() {
                    vm.regs.resize(dst + 1, Register::Null);
                }
                vm.regs[dst] = val;
            }

            // ── Rowid ────────────────────────────────────────────────────────
            // p1=cursor, p2=dst_reg
            Opcode::Rowid => {
                let slot = p1 as usize;
                let dst = p2 as usize;
                let rowid = vm.cursors.get_mut(slot)
                    .and_then(|c| c.as_mut())
                    .and_then(|c| c.rowid().ok())
                    .map(Register::Integer)
                    .unwrap_or(Register::Null);
                if dst >= vm.regs.len() {
                    vm.regs.resize(dst + 1, Register::Null);
                }
                vm.regs[dst] = rowid;
            }

            // ── MakeRecord ───────────────────────────────────────────────────
            // p1=first_reg, p2=count, p3=dst_reg
            // Stash the typed Vec<Register> for Insert to consume directly.
            Opcode::MakeRecord => {
                let first = p1 as usize;
                let count = p2 as usize;
                let dst = p3 as usize;
                if first + count > vm.regs.len() {
                    return Err(SQLITE_ERROR.into());
                }
                let record = vm.regs[first..first + count].to_vec();
                vm.pending_record = Some(record);
                // dst reg gets a sentinel so the register isn't left uninitialised.
                if dst >= vm.regs.len() {
                    vm.regs.resize(dst + 1, Register::Null);
                }
                vm.regs[dst] = Register::Integer(0); // sentinel
            }

            // ── NewRowid ─────────────────────────────────────────────────────
            // p1=cursor, p2=dst_reg
            Opcode::NewRowid => {
                let slot = p1 as usize;
                let dst = p2 as usize;
                let rowid = vm.cursors.get_mut(slot)
                    .and_then(|c| c.as_mut())
                    .ok_or(SQLITE_ERROR)?
                    .alloc_rowid()
                    .map_err(|_| SQLITE_ERROR)?;
                if dst >= vm.regs.len() {
                    vm.regs.resize(dst + 1, Register::Null);
                }
                vm.regs[dst] = Register::Integer(rowid);
            }

            // ── Insert ───────────────────────────────────────────────────────
            // p1=cursor, p2=record_reg (sentinel from MakeRecord), p3=rowid_reg
            Opcode::Insert => {
                let slot = p1 as usize;
                let rowid = vm.regs[p3 as usize].to_i64();
                let record = vm
                    .pending_record
                    .take()
                    .unwrap_or_else(|| vec![vm.regs[p2 as usize].clone()]);
                vm.cursors.get_mut(slot)
                    .and_then(|c| c.as_mut())
                    .ok_or(SQLITE_ERROR)?
                    .insert(rowid, &record)
                    .map_err(|_| SQLITE_ERROR)?;
                vm.changes_in_stmt += 1;
                vm.last_insert_rowid_in_stmt = rowid;
            }

            // ── Delete ───────────────────────────────────────────────────────
            // p1=cursor — deletes the row at the cursor's current position.
            // After delete, next() will naturally skip past the gap.
            Opcode::Delete => {
                let slot = p1 as usize;
                vm.cursors.get_mut(slot)
                    .and_then(|c| c.as_mut())
                    .ok_or(SQLITE_ERROR)?
                    .delete()
                    .map_err(|_| SQLITE_ERROR)?;
                vm.changes_in_stmt += 1;
            }

            // ── Limit ──────────────────────────────────────────────────────────
            // p1=limit_reg  p2=jump_when_exhausted  p3=offset_reg
            Opcode::Limit => {
                let offset_reg = p3 as usize;
                // Decrement offset first (skip rows).
                if offset_reg < vm.regs.len() {
                    let ov = vm.regs[offset_reg].to_i64();
                    if ov > 0 {
                        vm.regs[offset_reg] = Register::Integer(ov - 1);
                    }
                }
                // Decrement limit.
                let lv = reg!(p1).to_i64();
                if lv == 0 {
                    vm.pc = p2 as usize; // exhausted — jump to stop
                } else if lv > 0 {
                    reg!(p1) = Register::Integer(lv - 1);
                }
            }

            // ── DistinctCheck ──────────────────────────────────────────────────
            // p1=first_reg  p2=jump_if_dup  p3=count
            Opcode::DistinctCheck => {
                let first = p1 as usize;
                let count = p3 as usize;
                let max_needed = first + count;
                if max_needed > vm.regs.len() {
                    return Err(SQLITE_ERROR.into());
                }
                let key = row_key(&vm.regs[first..first + count]);
                if vm.distinct_seen.contains(&key) {
                    vm.pc = p2 as usize; // duplicate — jump past ResultRow
                } else {
                    vm.distinct_seen.insert(key);
                }
            }

            // ── DropTable ─────────────────────────────────────────────────────
            // p1=if_exists, p4=Text(table_name)
            Opcode::DropTable => {
                let if_exists = p1 != 0;
                let tname = match &instr.p4 {
                    P4::Text(s) => s.clone(),
                    _ => return Err(SQLITE_ERROR.into()),
                };
                match vm.backend.drop_table(&tname) {
                    Ok(_) => {}
                    Err(_) if if_exists => {}
                    Err(_) => return Err(SQLITE_ERROR.into()),
                }
            }

            // ── Aggregation: initialize accumulator state ────────────────────
            // p1=slot_reg  p4=FuncName(function_name)
            Opcode::AggInit => {
                let slot_reg = p1;
                let func = match &instr.p4 {
                    P4::FuncName(n) => n.clone(),
                    _ => String::new(),
                };
                let state = match func.as_str() {
                    "count" => AggState::Count(0),
                    "sum" => AggState::Sum(0.0, false),
                    "avg" => AggState::Avg(0.0, 0),
                    "min" => AggState::MinVal(None),
                    "max" => AggState::MaxVal(None),
                    "group_concat" | "string_agg" => AggState::GroupConcat(Vec::new(), None),
                    "total" => AggState::Total(0.0),
                    _ => AggState::Unset,
                };
                vm.agg_states.insert(slot_reg, state);
            }

            // ── Aggregation: accumulate value ─────────────────────────────────
            // p1=slot_reg  p2=value_reg
            Opcode::AggStep => {
                let slot_reg = p1;
                let val = reg!(p2).clone();
                let state = vm.agg_states.entry(slot_reg).or_insert(AggState::Count(0));
                match state {
                    AggState::Count(n) => {
                        if !val.is_null() {
                            *n += 1;
                        }
                    }
                    AggState::Sum(acc, seen) => {
                        if !val.is_null() {
                            *acc += val.to_f64();
                            *seen = true;
                        }
                    }
                    AggState::Avg(sum, cnt) => {
                        if !val.is_null() {
                            *sum += val.to_f64();
                            *cnt += 1;
                        }
                    }
                    AggState::MinVal(cur) => {
                        if !val.is_null() {
                            *cur = Some(match cur {
                                None => val,
                                Some(c) => {
                                    if val.cmp(&c) == Some(std::cmp::Ordering::Less) {
                                        val
                                    } else {
                                        c.clone()
                                    }
                                }
                            });
                        }
                    }
                    AggState::MaxVal(cur) => {
                        if !val.is_null() {
                            *cur = Some(match cur {
                                None => val,
                                Some(c) => {
                                    if val.cmp(&c) == Some(std::cmp::Ordering::Greater) {
                                        val
                                    } else {
                                        c.clone()
                                    }
                                }
                            });
                        }
                    }
                    AggState::GroupConcat(vals, _sep) => {
                        if !val.is_null() {
                            vals.push(val.to_text());
                        }
                    }
                    AggState::Total(acc) => {
                        if !val.is_null() {
                            *acc += val.to_f64();
                        }
                    }
                    AggState::Unset => {}
                }
            }

            // ── Aggregation: finalize and emit ────────────────────────────────
            // p1=slot_reg  p2=dst_reg
            Opcode::AggFinal => {
                let slot_reg = p1;
                let dst = p2 as usize;
                let result = match vm.agg_states.get(&slot_reg) {
                    None => Register::Null,
                    Some(AggState::Count(n)) => Register::Integer(*n),
                    Some(AggState::Sum(acc, seen)) => {
                        if *seen {
                            Register::Real(*acc)
                        } else {
                            Register::Null
                        }
                    }
                    Some(AggState::Avg(sum, cnt)) => {
                        if *cnt > 0 {
                            Register::Real(sum / *cnt as f64)
                        } else {
                            Register::Null
                        }
                    }
                    Some(AggState::MinVal(v)) => v.clone().unwrap_or(Register::Null),
                    Some(AggState::MaxVal(v)) => v.clone().unwrap_or(Register::Null),
                    Some(AggState::GroupConcat(vs, sep)) => {
                        let s = sep.as_deref().unwrap_or(",");
                        Register::Text(vs.join(s))
                    }
                    Some(AggState::Total(acc)) => Register::Real(*acc),
                    Some(AggState::Unset) => Register::Null,
                };
                vm.ensure_reg(dst);
                vm.regs[dst] = result;
            }

            // ── Legacy aggregate opcodes ──────────────────────────────────────
            // p1=counter_reg — increment by 1
            Opcode::AggCount => {
                let val = match &reg!(p1) {
                    Register::Integer(v) => v + 1,
                    _ => 1,
                };
                reg!(p1) = Register::Integer(val);
            }

            // p1=agg_reg  p2=dst_reg — copy aggregated value
            Opcode::AggValue => {
                reg!(p2) = reg!(p1).clone();
            }

            // ── SortRow: buffer a row for sorting ──────────────────────────────
            // p1=first_result_reg  p2=n_result  p3=first_key_reg  p4=Int(n_keys)
            Opcode::SortRow => {
                let first_result = p1 as usize;
                let n_result = p2 as usize;
                let first_key = p3 as usize;
                let n_keys = match &instr.p4 {
                    P4::Int(n) => *n as usize,
                    _ => 0,
                };

                let max_needed = (first_result + n_result).max(first_key + n_keys);
                if max_needed > vm.regs.len() {
                    vm.regs.resize(max_needed + 1, Register::Null);
                }

                if first_result + n_result > vm.regs.len() {
                    eprintln!(
                        "[rust-sqlite] SortRow ERROR: result range [{}, {}) exceeds {} regs",
                        first_result,
                        first_result + n_result,
                        vm.regs.len()
                    );
                    return Err(ExecError::RegisterOutOfBounds {
                        index: first_result + n_result,
                    });
                }
                if first_key + n_keys > vm.regs.len() {
                    eprintln!(
                        "[rust-sqlite] SortRow ERROR: key range [{}, {}) exceeds {} regs",
                        first_key,
                        first_key + n_keys,
                        vm.regs.len()
                    );
                    return Err(ExecError::RegisterOutOfBounds {
                        index: first_key + n_keys,
                    });
                }

                let result_cols = vm.regs[first_result..first_result + n_result].to_vec();
                let sort_keys = if n_keys > 0 {
                    vm.regs[first_key..first_key + n_keys].to_vec()
                } else {
                    result_cols.clone()
                };
                let tag_val = if first_key + n_keys < vm.regs.len() {
                    vm.regs[first_key + n_keys].to_i64() as i32
                } else {
                    0
                };

                vm.sort_buffer.push(SortEntry {
                    result_cols,
                    sort_keys,
                    tag: tag_val,
                });
            }

            // ── SortEmit: sort buffer and emit result rows ─────────────────────
            // p1=limit(-1=none)  p2=offset  p3=n_key_cols  p4=Text(dir_or_mode)
            Opcode::SortEmit => {
                let limit = p1;
                let offset = p2;
                let n_keys = p3 as usize;
                let mode = match &instr.p4 {
                    P4::Text(s) => s.clone(),
                    _ => String::new(),
                };

                let rows = std::mem::take(&mut vm.sort_buffer);
                let result_rows = apply_sort_emit(rows, n_keys, &mode, limit, offset);
                vm.result_rows = result_rows;
                vm.result_index = 0;
                // Emit the first row immediately so the caller sees SQLITE_ROW
                // before execution reaches Halt (which would return SQLITE_DONE).
                if !vm.result_rows.is_empty() {
                    vm.result_row = vm.result_rows[0].clone();
                    vm.result_index = 1;
                    return Ok(SQLITE_ROW);
                }
            }
            // ── Subprogram: execute a sub-program and merge rows ─────────────────────
            // p1=operator (0=Union, 1=UnionAll, 2=Intersect, 3=Except)
            // p4=Program(sub_prog)
            Opcode::Subprogram => {
                let sub_prog = match &instr.p4 {
                    P4::Program(p) => p.clone(),
                    _ => {
                        return Err(ExecError::Generic {
                            reason: "Subprogram: missing program".into(),
                        });
                    }
                };
                let op = p1;

                // Execute sub-program and collect its rows.
                // Clone the backend so the sub-VM sees the same live data.
                let mut sub_vm = Vm::new(256, sub_prog.n_vars, vm.backend.clone());
                sub_vm.bind_params = vm.bind_params.clone();

                let mut new_rows: Vec<Vec<Register>> = Vec::new();
                loop {
                    match step(&sub_prog, &mut sub_vm)? {
                        SQLITE_ROW => {
                            new_rows.push(sub_vm.result_row.clone());
                        }
                        SQLITE_DONE => break,
                        _ => break,
                    }
                }

                match op {
                    0 => {
                        // UNION — deduplicate
                        for row in new_rows {
                            let key = row_key(&row);
                            if !vm.distinct_seen.contains(&key) {
                                vm.distinct_seen.insert(key);
                                vm.result_rows.push(row);
                            }
                        }
                    }
                    1 => {
                        // UNION ALL — append all
                        vm.result_rows.extend(new_rows);
                    }
                    2 => {
                        // INTERSECT — keep only rows in both
                        let new_set: std::collections::HashSet<String> =
                            new_rows.iter().map(|r| row_key(r)).collect();
                        vm.result_rows.retain(|r| new_set.contains(&row_key(r)));
                    }
                    3 => {
                        // EXCEPT — remove rows that appear in right side
                        let new_set: std::collections::HashSet<String> =
                            new_rows.iter().map(|r| row_key(r)).collect();
                        vm.result_rows.retain(|r| !new_set.contains(&row_key(r)));
                    }
                    4 => {
                        // INSERT INTO ... SELECT: p2 = cursor slot, p3 = ncols
                        let cursor_slot = p2 as usize;
                        for row in new_rows {
                            if let Some(Some(c)) = vm.cursors.get_mut(cursor_slot) {
                                if let Ok(rowid) = c.alloc_rowid() {
                                    let _ = c.insert(rowid, &row);
                                    vm.last_insert_rowid_in_stmt = rowid;
                                    vm.changes_in_stmt += 1;
                                }
                            }
                        }
                    }
                    _ => {}
                }
                vm.result_index = 0;
            }

            // ── Union: signals end of all Subprogram branches ────────────────────────
            // Rows are already in vm.result_rows from Subprogram opcodes.
            // Mark halted so the drain loop at the top of step() serves them.
            Opcode::Union => {
                vm.halted = true;
                // result_rows already populated by preceding Subprogram opcodes.
                // The drain loop at the top of step() will serve them one by one.
                return Ok(if vm.result_rows.is_empty() {
                    SQLITE_DONE
                } else {
                    SQLITE_ROW
                });
            }

            // ── Index scan opcodes ────────────────────────────────────────────
            Opcode::SeekEq => {
                let slot = p1 as usize;
                let not_found_addr = p2 as usize;
                let key_val = vm.regs[p3 as usize].clone();
                let index_name = match &instr.p4 {
                    P4::Text(s) => s.clone(),
                    _ => return Err(ExecError::Generic { reason: "SeekEq: invalid index name".to_string() }),
                };

                let table_name = vm.cursors.get(slot)
                    .and_then(|c| c.as_ref())
                    .map(|c| c.table_name().to_string())
                    .ok_or(ExecError::CursorNotOpen { slot })?;

                // Look up index in global state.
                let rowids = {
                    let g = crate::wip_db::lock_global();
                    let entry = g.tables.get(&table_name)
                        .ok_or(ExecError::TableNotFound { table: table_name.clone() })?;
                    let idx = entry.indexes.get(&index_name)
                        .ok_or(ExecError::Generic { reason: format!("index {index_name} not found") })?;
                    let key = vec![OrdRegister(key_val)];
                    idx.entries.get(&key).cloned().unwrap_or_default()
                };

                unsafe { crate::src::src::vdbe::sqlite3_search_count += 2; }

                if rowids.is_empty() {
                    vm.pc = not_found_addr;
                } else {
                    // Position the cursor at the first matching rowid.
                    vm.cursors[slot].as_mut().unwrap()
                        .seek_rowid(rowids[0])
                        .map_err(|_| ExecError::Generic { reason: "SeekEq: seek_rowid failed".into() })?;
                    vm.index_states.insert(slot, (rowids, 0));
                }
            }

            Opcode::IdxNext => {
                let slot = p1 as usize;
                let loop_addr = p2 as usize;

                unsafe { crate::src::src::vdbe::sqlite3_search_count += 1; }

                // Advance index position and seek the cursor if there are more rows.
                let next_rowid = if let Some((rowids, pos)) = vm.index_states.get_mut(&slot) {
                    *pos += 1;
                    if *pos < rowids.len() {
                        Some(rowids[*pos])
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(rowid) = next_rowid {
                    vm.cursors.get_mut(slot)
                        .and_then(|c| c.as_mut())
                        .ok_or(ExecError::CursorNotOpen { slot })?
                        .seek_rowid(rowid)
                        .map_err(|e| ExecError::Generic { reason: format!("IdxNext: seek_rowid failed: {e}") })?;
                    vm.pc = loop_addr;
                }
            }
        }
    }
}

// ── Serialise a row slice to a string key (for distinct / sort dedup) ─────────

fn row_key(row: &[Register]) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    for (i, r) in row.iter().enumerate() {
        if i > 0 {
            s.push('\x1f');
        }
        match r {
            Register::Null => s.push_str("\x00N"),
            Register::Integer(v) => {
                let _ = write!(s, "\x01{v}");
            }
            Register::Real(v) => {
                let _ = write!(s, "\x02{v}");
            }
            Register::Text(t) => {
                s.push('\x03');
                s.push_str(t);
            }
            Register::Blob(b) => {
                let _ = write!(s, "\x04{}", hex_encode(b));
            }
        }
    }
    s
}

fn hex_encode(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02X}")).collect()
}

// ── Sort/emit logic ───────────────────────────────────────────────────────────

fn apply_sort_emit(
    mut rows: Vec<SortEntry>,
    n_keys: usize,
    mode: &str,
    limit: i32,
    offset: i32,
) -> Vec<Vec<Register>> {
    // Determine mode.
    let is_group_by = mode.starts_with("G:");
    let is_union_all = mode == "UA";
    let is_union = mode == "U";
    let is_intersect = mode == "I";
    let is_except = mode == "E";
    let is_compound = is_union || is_union_all || is_intersect || is_except;

    // For GROUP BY mode: "G:" + sort_dir_chars + ":" + comma_funcs
    // split_dirs = chars before first ':', split_funcs = after first ':'
    let dir_part = if is_group_by { &mode[2..] } else { mode };
    let (sort_dirs_part, funcs_part) = if is_group_by {
        if let Some(colon) = dir_part.find(':') {
            (&dir_part[..colon], &dir_part[colon + 1..])
        } else {
            (dir_part, "")
        }
    } else {
        (dir_part, dir_part)
    };

    // Sort by sort_keys.
    if n_keys > 0 || is_group_by {
        let dirs: Vec<bool> = sort_dirs_part
            .chars()
            .filter(|&c| c == 'A' || c == 'D')
            .map(|c| c == 'D')
            .collect();
        rows.sort_by(|a, b| {
            let ka = &a.sort_keys;
            let kb = &b.sort_keys;
            for i in 0..ka.len().max(kb.len()) {
                let desc = dirs.get(i).copied().unwrap_or(false);
                let va = ka.get(i).unwrap_or(&Register::Null);
                let vb = kb.get(i).unwrap_or(&Register::Null);
                let ord = va.compare(vb).unwrap_or(std::cmp::Ordering::Equal);
                let ord = if desc { ord.reverse() } else { ord };
                if ord != std::cmp::Ordering::Equal {
                    return ord;
                }
            }
            std::cmp::Ordering::Equal
        });
    }

    let mut result: Vec<Vec<Register>> = Vec::new();

    if is_compound {
        // Set operations: split by tag.
        let left: Vec<_> = rows.iter().filter(|r| r.tag == 0).collect();
        let right: Vec<_> = rows.iter().filter(|r| r.tag != 0).collect();

        if is_union_all {
            result.extend(left.iter().map(|r| r.result_cols.clone()));
            result.extend(right.iter().map(|r| r.result_cols.clone()));
        } else if is_union {
            let mut seen = std::collections::HashSet::new();
            for r in left.iter().chain(right.iter()) {
                let k = row_key(&r.result_cols);
                if seen.insert(k) {
                    result.push(r.result_cols.clone());
                }
            }
        } else if is_intersect {
            let right_keys: std::collections::HashSet<_> =
                right.iter().map(|r| row_key(&r.result_cols)).collect();
            let mut seen = std::collections::HashSet::new();
            for r in &left {
                let k = row_key(&r.result_cols);
                if right_keys.contains(&k) && seen.insert(k.clone()) {
                    result.push(r.result_cols.clone());
                }
            }
        } else if is_except {
            let right_keys: std::collections::HashSet<_> =
                right.iter().map(|r| row_key(&r.result_cols)).collect();
            let mut seen = std::collections::HashSet::new();
            for r in &left {
                let k = row_key(&r.result_cols);
                if !right_keys.contains(&k) && seen.insert(k.clone()) {
                    result.push(r.result_cols.clone());
                }
            }
        }
    } else if is_group_by {
        // Group by n_keys leading sort-key columns, aggregate the rest.
        // agg_funcs[i] is the function name for result column i (maps 1:1).
        let agg_funcs: Vec<&str> = if funcs_part.is_empty() {
            vec![]
        } else {
            funcs_part.split(',').collect()
        };
        let n_result = rows.first().map(|r| r.result_cols.len()).unwrap_or(0);
        let n_group = n_keys;

        // Bucket rows by group key.
        let mut groups: Vec<(Vec<Register>, Vec<Vec<Register>>)> = Vec::new();
        let mut key_index: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for row in &rows {
            let group_key = &row.sort_keys[..n_group.min(row.sort_keys.len())];
            let k = row_key(group_key);
            if let Some(&idx) = key_index.get(&k) {
                groups[idx].1.push(row.result_cols.clone());
            } else {
                key_index.insert(k, groups.len());
                groups.push((group_key.to_vec(), vec![row.result_cols.clone()]));
            }
        }

        for (group_key, group_rows) in groups {
            let mut out_row = Vec::with_capacity(n_result);
            for col_idx in 0..n_result {
                let func = agg_funcs.get(col_idx).copied().unwrap_or("_");
                let col_vals: Vec<&Register> = group_rows
                    .iter()
                    .map(|r| r.get(col_idx).unwrap_or(&Register::Null))
                    .collect();
                let v = apply_group_agg(func, &col_vals, &group_key, col_idx, n_group);
                out_row.push(v);
            }
            result.push(out_row);
        }
    } else {
        // Plain ORDER BY.
        result.extend(rows.into_iter().map(|r| r.result_cols));
    }

    // Apply OFFSET then LIMIT.
    let start = (offset as usize).min(result.len());
    let result = result.into_iter().skip(start);
    let result: Vec<_> = if limit >= 0 {
        result.take(limit as usize).collect()
    } else {
        result.collect()
    };
    result
}

fn apply_group_agg(
    func: &str,
    vals: &[&Register],
    group_key: &[Register],
    col_idx: usize,
    n_group: usize,
) -> Register {
    use Register::*;
    // Non-aggregate columns ("_") return the first value from the group.
    // All rows in a group share the same value for non-aggregate columns.
    if func == "_" {
        return vals.first().copied().cloned().unwrap_or(Null);
    }
    // Suppress unused-variable warnings for params no longer needed for early return.
    let _ = (col_idx, n_group, group_key);

    // Check for DISTINCT marker (e.g., "count#d" instead of "count")
    let (func_name, is_distinct) = if let Some(stripped) = func.strip_suffix("#d") {
        (stripped, true)
    } else {
        (func, false)
    };

    let non_null: Vec<&Register> = vals.iter().copied().filter(|r| !r.is_null()).collect();

    match func_name {
        "count" => {
            if is_distinct {
                // COUNT(DISTINCT x): count unique non-null values
                let mut seen = std::collections::HashSet::new();
                for val in &non_null {
                    seen.insert(row_key(&[(*val).clone()]));
                }
                Integer(seen.len() as i64)
            } else {
                Integer(non_null.len() as i64)
            }
        }
        "sum" => {
            if non_null.is_empty() {
                return Null;
            }
            if is_distinct {
                // SUM(DISTINCT x): sum unique values
                let mut seen = std::collections::HashSet::new();
                let mut sum = 0.0;
                for val in &non_null {
                    let k = row_key(&[(*val).clone()]);
                    if seen.insert(k) {
                        sum += val.to_f64();
                    }
                }
                Real(sum)
            } else {
                Real(non_null.iter().map(|r| r.to_f64()).sum())
            }
        }
        "avg" => {
            if non_null.is_empty() {
                return Null;
            }
            if is_distinct {
                // AVG(DISTINCT x): average of unique values
                let mut seen = std::collections::HashSet::new();
                let mut sum = 0.0;
                let mut count = 0;
                for val in &non_null {
                    let k = row_key(&[(*val).clone()]);
                    if seen.insert(k) {
                        sum += val.to_f64();
                        count += 1;
                    }
                }
                if count > 0 {
                    Real(sum / count as f64)
                } else {
                    Null
                }
            } else {
                Real(non_null.iter().map(|r| r.to_f64()).sum::<f64>() / non_null.len() as f64)
            }
        }
        "min" => non_null
            .iter()
            .min_by(|a, b| a.compare(b).unwrap_or(std::cmp::Ordering::Equal))
            .copied()
            .cloned()
            .unwrap_or(Null),
        "max" => non_null
            .iter()
            .max_by(|a, b| a.compare(b).unwrap_or(std::cmp::Ordering::Equal))
            .copied()
            .cloned()
            .unwrap_or(Null),
        "group_concat" => {
            let strs: Vec<String> = if is_distinct {
                // GROUP_CONCAT(DISTINCT x): concat unique values
                let mut seen = std::collections::HashSet::new();
                let mut result = Vec::new();
                for val in &non_null {
                    let k = row_key(&[(*val).clone()]);
                    if seen.insert(k) {
                        result.push(val.to_text());
                    }
                }
                result
            } else {
                non_null.iter().map(|r| r.to_text()).collect()
            };
            if strs.is_empty() {
                Null
            } else {
                Text(strs.join(","))
            }
        }
        "total" => Real(non_null.iter().map(|r| r.to_f64()).sum()),
        _ => Null,
    }
}

// ── Built-in scalar functions ─────────────────────────────────────────────────

fn call_builtin(name: &str, args: &[Register]) -> Register {
    use Register::*;
    macro_rules! arg {
        ($n:expr) => {
            args.get($n).unwrap_or(&Null)
        };
    }

    match name {
        "abs" => match arg!(0) {
            Integer(v) => Integer(v.abs()),
            Real(v) => Real(v.abs()),
            Null => Null,
            _ => Null,
        },
        "length" => match arg!(0) {
            Text(s) => Integer(s.chars().count() as i64),
            Blob(b) => Integer(b.len() as i64),
            Null => Null,
            _ => Null,
        },
        "upper" => match arg!(0) {
            Text(s) => Text(s.to_uppercase()),
            Null => Null,
            _ => Null,
        },
        "lower" => match arg!(0) {
            Text(s) => Text(s.to_lowercase()),
            Null => Null,
            _ => Null,
        },
        "typeof" => Text(
            match arg!(0) {
                Null => "null",
                Integer(_) => "integer",
                Real(_) => "real",
                Text(_) => "text",
                Blob(_) => "blob",
            }
            .into(),
        ),
        "hex" => match arg!(0) {
            Blob(b) => Text(b.iter().map(|byte| format!("{byte:02X}")).collect()),
            Null => Null,
            _ => Null,
        },
        "coalesce" => args.iter().find(|r| !r.is_null()).cloned().unwrap_or(Null),
        "ifnull" | "nvl" => {
            if arg!(0).is_null() {
                arg!(1).clone()
            } else {
                arg!(0).clone()
            }
        }
        "nullif" => {
            if args.len() >= 2 {
                match arg!(0).cmp(arg!(1)) {
                    Some(std::cmp::Ordering::Equal) => Null,
                    _ => arg!(0).clone(),
                }
            } else {
                Null
            }
        }
        "iif" | "if" => {
            if args.len() >= 3 {
                if arg!(0).is_truthy() {
                    arg!(1).clone()
                } else {
                    arg!(2).clone()
                }
            } else {
                Null
            }
        }
        "max" => args
            .iter()
            .filter(|r| !r.is_null())
            .max_by(|a, b| a.cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .cloned()
            .unwrap_or(Null),
        "min" => args
            .iter()
            .filter(|r| !r.is_null())
            .min_by(|a, b| a.cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .cloned()
            .unwrap_or(Null),
        "sign" => match arg!(0) {
            Integer(v) => Integer(v.signum()),
            Real(v) => Integer(v.signum() as i64),
            Null => Null,
            _ => Null,
        },
        "round" => match arg!(0) {
            Integer(v) => Integer(*v),
            Real(v) => {
                let places = args.get(1).map(|r| r.to_i64()).unwrap_or(0).max(0) as u32;
                let factor = 10f64.powi(places as i32);
                Real((v * factor).round() / factor)
            }
            Null => Null,
            _ => Null,
        },
        // ── BATCH 1: Math Functions (11) ──────────────────────────────────────
        "ceil" | "ceiling" => match arg!(0) {
            Integer(v) => Integer(*v),
            Real(v) => Real(v.ceil()),
            Null => Null,
            _ => Null,
        },
        "floor" => match arg!(0) {
            Integer(v) => Integer(*v),
            Real(v) => Real(v.floor()),
            Null => Null,
            _ => Null,
        },
        "sqrt" => match arg!(0) {
            Integer(v) => Real((*v as f64).sqrt()),
            Real(v) => Real(v.sqrt()),
            Null => Null,
            _ => Null,
        },
        "exp" => Real(arg!(0).to_f64().exp()),
        "log" => {
            let x = arg!(0).to_f64();
            if x <= 0.0 { Null } else { Real(x.ln()) }
        }
        "log10" => {
            let x = arg!(0).to_f64();
            if x <= 0.0 { Null } else { Real(x.log10()) }
        }
        "log2" => {
            let x = arg!(0).to_f64();
            if x <= 0.0 { Null } else { Real(x.log2()) }
        }
        "pow" | "power" => {
            let base = arg!(0).to_f64();
            let exp = arg!(1).to_f64();
            Real(base.powf(exp))
        }
        "sin" => Real(arg!(0).to_f64().sin()),
        "cos" => Real(arg!(0).to_f64().cos()),
        "tan" => Real(arg!(0).to_f64().tan()),
        // ── BATCH 2: Inverse Trig + Conversions + String (11) ─────────────────
        "asin" => Real(arg!(0).to_f64().asin()),
        "acos" => Real(arg!(0).to_f64().acos()),
        "atan" => Real(arg!(0).to_f64().atan()),
        "atan2" => Real(arg!(0).to_f64().atan2(arg!(1).to_f64())),
        "degrees" => Real(arg!(0).to_f64().to_degrees()),
        "radians" => Real(arg!(0).to_f64().to_radians()),
        "pi" => Real(std::f64::consts::PI),
        "len" => match arg!(0) {
            Text(s) => Integer(s.chars().count() as i64),
            Blob(b) => Integer(b.len() as i64),
            Null => Null,
            _ => Null,
        },
        "ltrim" => {
            if arg!(0).is_null() {
                return Null;
            }
            let s = arg!(0).to_text();
            let chars = args.get(1).map(|r| r.to_text());
            let result = match chars {
                None => s.trim_start().to_string(),
                Some(c) => {
                    let set: Vec<char> = c.chars().collect();
                    s.trim_start_matches(|ch| set.contains(&ch)).to_string()
                }
            };
            Text(result)
        }
        "rtrim" => {
            if arg!(0).is_null() {
                return Null;
            }
            let s = arg!(0).to_text();
            let chars = args.get(1).map(|r| r.to_text());
            let result = match chars {
                None => s.trim_end().to_string(),
                Some(c) => {
                    let set: Vec<char> = c.chars().collect();
                    s.trim_end_matches(|ch| set.contains(&ch)).to_string()
                }
            };
            Text(result)
        }
        "trim" => {
            let s = arg!(0).to_text();
            let chars = args.get(1).map(|r| r.to_text());
            let result = match chars {
                None => s.trim().to_string(),
                Some(c) => {
                    let set: Vec<char> = c.chars().collect();
                    s.trim_matches(|ch| set.contains(&ch)).to_string()
                }
            };
            if arg!(0).is_null() {
                Null
            } else {
                Text(result)
            }
        }
        // ── BATCH 3: String Functions (11) ────────────────────────────────────
        "substr" | "substring" => {
            if arg!(0).is_null() {
                return Null;
            }
            let s: Vec<char> = arg!(0).to_text().chars().collect();
            let start = arg!(1).to_i64();
            let len = args.get(2).map(|r| r.to_i64());
            let start_idx = if start > 0 {
                (start as usize).saturating_sub(1)
            } else if start < 0 {
                let from_end = (-start) as usize;
                if from_end > s.len() {
                    0
                } else {
                    s.len() - from_end
                }
            } else {
                0
            };
            let slice = &s[start_idx.min(s.len())..];
            let result: String = match len {
                None => slice.iter().collect(),
                Some(l) => {
                    if l < 0 {
                        String::new()
                    } else {
                        slice.iter().take(l as usize).collect()
                    }
                }
            };
            Text(result)
        }
        "instr" => {
            if arg!(0).is_null() || arg!(1).is_null() {
                return Null;
            }
            let haystack = arg!(0).to_text();
            let needle = arg!(1).to_text();
            if needle.is_empty() {
                return Integer(0);
            }
            let pos = haystack
                .find(&needle)
                .map(|b| haystack[..b].chars().count() as i64 + 1)
                .unwrap_or(0);
            Integer(pos)
        }
        "replace" => {
            if arg!(0).is_null() {
                return Null;
            }
            let text = arg!(1).to_text();
            let from = arg!(2).to_text();
            let to = args.get(3).map(|r| r.to_text()).unwrap_or_default();
            Text(text.replace(&from, &to))
        }
        "lpad" => {
            if arg!(0).is_null() {
                return Null;
            }
            let s = arg!(0).to_text();
            let len = arg!(1).to_i64() as usize;
            let pad = args
                .get(2)
                .map(|r| r.to_text())
                .unwrap_or_else(|| " ".to_string());
            if pad.is_empty() {
                return Text(s);
            }
            if s.len() >= len {
                return Text(s);
            }
            let needed = len - s.len();
            let repeat_count = (needed + pad.len() - 1) / pad.len();
            let padding: String = pad.repeat(repeat_count).chars().take(needed).collect();
            Text(format!("{}{}", padding, s))
        }
        "rpad" => {
            if arg!(0).is_null() {
                return Null;
            }
            let s = arg!(0).to_text();
            let len = arg!(1).to_i64() as usize;
            let pad = args
                .get(2)
                .map(|r| r.to_text())
                .unwrap_or_else(|| " ".to_string());
            if pad.is_empty() {
                return Text(s);
            }
            if s.len() >= len {
                return Text(s);
            }
            let needed = len - s.len();
            let repeat_count = (needed + pad.len() - 1) / pad.len();
            let padding: String = pad.repeat(repeat_count).chars().take(needed).collect();
            Text(format!("{}{}", s, padding))
        }
        "quote" => match arg!(0) {
            Null => Text("NULL".to_string()),
            Integer(v) => Text(v.to_string()),
            Real(v) => Text(v.to_string()),
            Text(s) => Text(format!("'{}'", s.replace("'", "''"))),
            Blob(b) => Text(format!(
                "X'{}'",
                b.iter()
                    .map(|byte| format!("{:02X}", byte))
                    .collect::<String>()
            )),
        },
        "repeat" => {
            if arg!(0).is_null() {
                return Null;
            }
            let s = arg!(0).to_text();
            let count = arg!(1).to_i64().max(0) as usize;
            Text(s.repeat(count))
        }
        "reverse" => {
            if arg!(0).is_null() {
                return Null;
            }
            let s = arg!(0).to_text();
            Text(s.chars().rev().collect())
        }
        "unicode" => {
            if arg!(0).is_null() {
                return Null;
            }
            let s = arg!(0).to_text();
            match s.chars().next() {
                Some(c) => Integer(c as u32 as i64),
                None => Null,
            }
        }
        // ── BATCH 4: Special & Date Functions (11) ───────────────────────────
        "unhex" => {
            if arg!(0).is_null() {
                return Null;
            }
            let s = arg!(0).to_text().to_uppercase();
            let mut bytes = Vec::new();
            let mut chars = s.chars();
            while let (Some(h), Some(l)) = (chars.next(), chars.next()) {
                if let (Some(hv), Some(lv)) = (h.to_digit(16), l.to_digit(16)) {
                    bytes.push(((hv << 4) | lv) as u8);
                } else {
                    return Null;
                }
            }
            Blob(bytes)
        }
        "soundex" => {
            if arg!(0).is_null() {
                return Null;
            }
            let s = arg!(0).to_text().to_uppercase();
            if s.is_empty() {
                return Text("0000".to_string());
            }
            let mut first_chars = s.chars();
            let first = first_chars.next().ok_or_else(|| Text("0000".to_string())); // Should not happen due to is_empty check
            let first = match first {
                Ok(c) => c,
                Err(res) => return res,
            };
            let mut code = first.to_string();
            let soundex_code = |c: char| -> Option<char> {
                match c {
                    'B' | 'F' | 'P' | 'V' => Some('1'),
                    'C' | 'G' | 'J' | 'K' | 'Q' | 'S' | 'X' | 'Z' => Some('2'),
                    'D' | 'T' => Some('3'),
                    'L' => Some('4'),
                    'M' | 'N' => Some('5'),
                    'R' => Some('6'),
                    _ => None,
                }
            };
            let mut last = soundex_code(first);
            for c in s.chars().skip(1) {
                if let Some(digit) = soundex_code(c) {
                    if Some(digit) != last && digit != '0' {
                        code.push(digit);
                        if code.len() == 4 {
                            break;
                        }
                    }
                    last = Some(digit);
                } else {
                    last = None;
                }
            }
            while code.len() < 4 {
                code.push('0');
            }
            Text(code)
        }
        "char" => {
            let mut result = String::new();
            for i in 0..args.len() {
                let code = arg!(i).to_i64();
                if code >= 0 && code <= 0x10FFFF {
                    if let Some(c) = char::from_u32(code as u32) {
                        result.push(c);
                    }
                }
            }
            Text(result)
        }
        "randomblob" => {
            let len = arg!(0).to_i64().max(0) as usize;
            let mut bytes = vec![0u8; len];
            for b in &mut bytes {
                *b = (std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| {
                        (d.as_nanos() as u64)
                            .wrapping_mul(1103515245)
                            .wrapping_add(12345)
                    })
                    .unwrap_or(0)
                    & 0xFF) as u8;
            }
            Blob(bytes)
        }
        "zeroblob" => {
            let len = arg!(0).to_i64().max(0) as usize;
            Blob(vec![0u8; len])
        }
        "format" => {
            if arg!(0).is_null() {
                return Null;
            }
            let fmt = arg!(0).to_text();
            let mut result = String::new();
            let mut arg_idx = 1;
            let mut chars = fmt.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '%' {
                    if let Some(&next) = chars.peek() {
                        chars.next();
                        match next {
                            '%' => result.push('%'),
                            's' => {
                                if arg_idx < args.len() {
                                    result.push_str(&args[arg_idx].to_text());
                                    arg_idx += 1;
                                }
                            }
                            'd' | 'i' => {
                                if arg_idx < args.len() {
                                    result.push_str(&args[arg_idx].to_i64().to_string());
                                    arg_idx += 1;
                                }
                            }
                            _ => {
                                result.push('%');
                                result.push(next);
                            }
                        }
                    }
                } else {
                    result.push(c);
                }
            }
            Text(result)
        }
        "date" | "time" | "julianday" | "now" | "strftime" | "unixepoch" => {
            // Stub implementations returning current time-related values
            match name {
                "date" => {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    let days = now / 86400;
                    let year = 1970 + days / 365;
                    let month = ((days % 365) / 30) + 1;
                    let day = ((days % 365) % 30) + 1;
                    Text(format!("{:04}-{:02}-{:02}", year, month, day))
                }
                "time" => Text("00:00:00".to_string()),
                "julianday" => Real(2440588.0), // Epoch JD
                "now" => Text("2026-03-03T12:00:00Z".to_string()),
                "strftime" => Text(String::new()), // Stub
                "unixepoch" => {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0);
                    Integer(now)
                }
                _ => Null,
            }
        }
        _ => Null,
    }
}
