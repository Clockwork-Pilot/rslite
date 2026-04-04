//! Comparison tests: sqlite3_str_vappendf2_args (new VaList-free formatter)
//! vs sqlite3_mprintf (old formatter, reference).
//!
//! Each test constructs PrintfArg slices manually, calls the new formatter,
//! and asserts the output matches what sqlite3_mprintf produces.

use core::ffi::{c_char, c_int, c_void};
use std::ffi::CStr;

use sqlite_noamalgam::src::src::printf::{
    PrintfArg, sqlite3_str_vappendf2_args,
    sqlite3StrAccumInit, sqlite3StrAccumFinish,
};
use sqlite_noamalgam::src::headers::sqliteInt_h::StrAccum;
use sqlite_noamalgam::sqliteLimit_h::SQLITE_MAX_LENGTH;

unsafe extern "C" {
    fn sqlite3_mprintf(fmt: *const c_char, ...) -> *mut c_char;
    fn sqlite3_free(p: *mut c_void);
}

/// Call sqlite3_str_vappendf2_args with format + args, return result as String.
/// Uses a large stack buffer with mxAlloc=0 to avoid db-allocator issues.
unsafe fn fmt2(fmt: &[u8], args: &[PrintfArg]) -> String {
    let mut zBase: [c_char; 8192] = [0; 8192];
    let mut acc: StrAccum = core::mem::zeroed();
    sqlite3StrAccumInit(
        &raw mut acc,
        core::ptr::null_mut(),
        &raw mut zBase as *mut c_char,
        8192,
        0, // mxAlloc=0: no dynamic growth, output truncated at 8192
    );
    sqlite3_str_vappendf2_args(&raw mut acc, fmt.as_ptr() as _, args);
    // NUL-terminate
    if !acc.zText.is_null() && (acc.nChar as usize) < 8192 {
        *acc.zText.offset(acc.nChar as isize) = 0;
    }
    if acc.zText.is_null() || acc.nChar == 0 {
        return String::new();
    }
    CStr::from_ptr(acc.zText).to_string_lossy().into_owned()
}

/// Call sqlite3_mprintf (reference), return result as String.
unsafe fn mprintf_ref(p: *mut c_char) -> String {
    if p.is_null() { return "(null)".into(); }
    let s = CStr::from_ptr(p).to_string_lossy().into_owned();
    sqlite3_free(p as *mut c_void);
    s
}

// ═══ %d — signed decimal integers ═══════════════════════════════════════

#[test]
fn cmp_d_basic() {
    unsafe {
        for &v in &[0i64, 1, -1, 42, -42, 2147483647, -2147483648] {
            let new = fmt2(b"%d\0", &[PrintfArg::Int(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%d\0".as_ptr() as _, v as i32));
            assert_eq!(new, old, "%d({v})");
        }
    }
}

#[test]
fn cmp_d_zero_pad() {
    unsafe {
        for &v in &[0i64, 5, 42, 2024] {
            let new = fmt2(b"%04d\0", &[PrintfArg::Int(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%04d\0".as_ptr() as _, v as i32));
            assert_eq!(new, old, "%04d({v})");
        }
        for &v in &[0i64, 5, 9, 10, 99] {
            let new = fmt2(b"%02d\0", &[PrintfArg::Int(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%02d\0".as_ptr() as _, v as i32));
            assert_eq!(new, old, "%02d({v})");
        }
    }
}

#[test]
fn cmp_d_negative_pad() {
    unsafe {
        for &v in &[-1i64, -42, -999] {
            let new = fmt2(b"%06d\0", &[PrintfArg::Int(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%06d\0".as_ptr() as _, v as i32));
            assert_eq!(new, old, "%06d({v})");
        }
    }
}

#[test]
fn cmp_d_width_right_align() {
    unsafe {
        for &v in &[0i64, 5, 42, -1] {
            let new = fmt2(b"%6d\0", &[PrintfArg::Int(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%6d\0".as_ptr() as _, v as i32));
            assert_eq!(new, old, "%6d({v})");
        }
    }
}

#[test]
fn cmp_d_width_left_align() {
    unsafe {
        for &v in &[0i64, 5, 42, -1] {
            let new = fmt2(b"%-6d\0", &[PrintfArg::Int(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%-6d\0".as_ptr() as _, v as i32));
            assert_eq!(new, old, "%-6d({v})");
        }
    }
}

// ═══ %u — unsigned decimal ══════════════════════════════════════════════

#[test]
fn cmp_u_basic() {
    unsafe {
        for &v in &[0u64, 1, 42, 4294967295] {
            let new = fmt2(b"%u\0", &[PrintfArg::UInt(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%u\0".as_ptr() as _, v as u32));
            assert_eq!(new, old, "%u({v})");
        }
    }
}

// ═══ %lld — 64-bit signed ══════════════════════════════════════════════

#[test]
fn cmp_lld_basic() {
    unsafe {
        for &v in &[0i64, 1, -1, i64::MAX, i64::MIN] {
            let new = fmt2(b"%lld\0", &[PrintfArg::Int(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%lld\0".as_ptr() as _, v));
            assert_eq!(new, old, "%lld({v})");
        }
    }
}

// ═══ %x / %X — hexadecimal ═════════════════════════════════════════════

#[test]
fn cmp_x_basic() {
    unsafe {
        for &v in &[0u64, 1, 0xAB, 0xDEAD, 0xFFFFFFFF] {
            let new = fmt2(b"%x\0", &[PrintfArg::UInt(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%x\0".as_ptr() as _, v as u32));
            assert_eq!(new, old, "%x({v:#x})");
        }
    }
}

#[test]
fn cmp_x_upper() {
    unsafe {
        for &v in &[0u64, 0xDEAD, 0xFFFF] {
            let new = fmt2(b"%X\0", &[PrintfArg::UInt(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%X\0".as_ptr() as _, v as u32));
            assert_eq!(new, old, "%X({v:#x})");
        }
    }
}

#[test]
fn cmp_x_zero_pad() {
    unsafe {
        for &v in &[0u64, 5, 0x0F, 0xFF] {
            let new = fmt2(b"%02x\0", &[PrintfArg::UInt(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%02x\0".as_ptr() as _, v as u32));
            assert_eq!(new, old, "%02x({v:#x})");
        }
        // %06X with larger values
        for &v in &[0xABCu64, 0x123456] {
            let new = fmt2(b"%06X\0", &[PrintfArg::UInt(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%06X\0".as_ptr() as _, v as u32));
            assert_eq!(new, old, "%06X({v:#x})");
        }
    }
}

#[test]
fn cmp_x_precision() {
    unsafe {
        // "%.3x+%.6x" pattern from codebase
        let new = fmt2(b"%.3x+%.6x\0", &[PrintfArg::UInt(0xAB), PrintfArg::UInt(0x123456)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%.3x+%.6x\0".as_ptr() as _, 0xABu32, 0x123456u32));
        assert_eq!(new, old, "%.3x+%.6x");
    }
}

// ═══ %s — string ════════════════════════════════════════════════════════

#[test]
fn cmp_s_basic() {
    unsafe {
        let hello = b"hello\0".as_ptr() as *mut c_char;
        let world = b"world\0".as_ptr() as *mut c_char;

        let new = fmt2(b"%s\0", &[PrintfArg::Str(hello)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%s\0".as_ptr() as _, hello));
        assert_eq!(new, old, "%s");

        let new = fmt2(b"%s %s\0", &[PrintfArg::Str(hello), PrintfArg::Str(world)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%s %s\0".as_ptr() as _, hello, world));
        assert_eq!(new, old, "%s %s");
    }
}

#[test]
fn cmp_s_null() {
    unsafe {
        let new = fmt2(b"%s\0", &[PrintfArg::Str(core::ptr::null_mut())]);
        let old = mprintf_ref(sqlite3_mprintf(b"%s\0".as_ptr() as _, core::ptr::null::<c_char>()));
        assert_eq!(new, old, "%s(NULL)");
    }
}

#[test]
fn cmp_s_precision() {
    unsafe {
        let data = b"hello world\0".as_ptr() as *mut c_char;
        for &n in &[0i32, 3, 5, 11] {
            let new = fmt2(b"%.*s\0", &[PrintfArg::Int(n as i64), PrintfArg::Str(data)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%.*s\0".as_ptr() as _, n, data));
            assert_eq!(new, old, "%.*s(n={n})");
        }
    }
}

// ═══ %c — character ═════════════════════════════════════════════════════

#[test]
fn cmp_c_basic() {
    unsafe {
        for &ch in &[b'A' as u32, b'z' as u32, b'+' as u32, b' ' as u32] {
            let new = fmt2(b"%c\0", &[PrintfArg::Char(ch)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%c\0".as_ptr() as _, ch as i32));
            assert_eq!(new, old, "%c({ch})");
        }
    }
}

#[test]
fn cmp_c_precision() {
    // %.4c with char 0 — master journal padding
    unsafe {
        let new_bytes = {
            let mut zBase: [c_char; 512] = [0; 512];
            let mut acc: StrAccum = core::mem::zeroed();
            sqlite3StrAccumInit(
                &raw mut acc,
                core::ptr::null_mut(),
                &raw mut zBase as *mut c_char,
                512,
                0, // mxAlloc=0
            );
            sqlite3_str_vappendf2_args(
                &raw mut acc,
                b"%.4c%s%.16c\0".as_ptr() as _,
                &[
                    PrintfArg::Char(0),
                    PrintfArg::Str(b"test.db\0".as_ptr() as *mut c_char),
                    PrintfArg::Char(0),
                ],
            );
            let nChar = acc.nChar as usize;
            let mut v = vec![0u8; nChar];
            core::ptr::copy_nonoverlapping(acc.zText as *const u8, v.as_mut_ptr(), nChar);
            v
        };
        // Expected: 4 NULs + "test.db" + 16 NULs
        assert_eq!(&new_bytes[0..4], &[0, 0, 0, 0], "%.4c prefix");
        assert_eq!(&new_bytes[4..11], b"test.db", "filename");
        assert_eq!(&new_bytes[11..27], &[0u8; 16], "%.16c suffix");
    }
}

// ═══ %q / %Q / %w — SQL escaping ═══════════════════════════════════════

#[test]
fn cmp_q_basic() {
    unsafe {
        let simple = b"hello\0".as_ptr() as *mut c_char;
        let apost = b"it's\0".as_ptr() as *mut c_char;

        let new = fmt2(b"%q\0", &[PrintfArg::Str(simple)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%q\0".as_ptr() as _, simple));
        assert_eq!(new, old, "%q simple");

        let new = fmt2(b"%q\0", &[PrintfArg::Str(apost)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%q\0".as_ptr() as _, apost));
        assert_eq!(new, old, "%q apostrophe");
    }
}

#[test]
fn cmp_Q_basic() {
    unsafe {
        let simple = b"hello\0".as_ptr() as *mut c_char;
        let apost = b"it's\0".as_ptr() as *mut c_char;
        let null_ptr = core::ptr::null_mut::<c_char>();

        let new = fmt2(b"%Q\0", &[PrintfArg::Str(simple)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%Q\0".as_ptr() as _, simple));
        assert_eq!(new, old, "%Q simple");

        let new = fmt2(b"%Q\0", &[PrintfArg::Str(apost)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%Q\0".as_ptr() as _, apost));
        assert_eq!(new, old, "%Q apostrophe");

        let new = fmt2(b"%Q\0", &[PrintfArg::Str(null_ptr)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%Q\0".as_ptr() as _, null_ptr));
        assert_eq!(new, old, "%Q NULL");
    }
}

#[test]
fn cmp_w_basic() {
    unsafe {
        let simple = b"main\0".as_ptr() as *mut c_char;
        let with_dq = b"my\"table\0".as_ptr() as *mut c_char;

        let new = fmt2(b"%w\0", &[PrintfArg::Str(simple)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%w\0".as_ptr() as _, simple));
        assert_eq!(new, old, "%w simple");

        let new = fmt2(b"%w\0", &[PrintfArg::Str(with_dq)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%w\0".as_ptr() as _, with_dq));
        assert_eq!(new, old, "%w with double-quote");

        // The pattern from SQLite internals: "%w".%s
        let tbl = b"sqlite_master\0".as_ptr() as *mut c_char;
        let new = fmt2(
            b"\"%w\".%s\0",
            &[PrintfArg::Str(simple), PrintfArg::Str(tbl)],
        );
        let old = mprintf_ref(sqlite3_mprintf(b"\"%w\".%s\0".as_ptr() as _, simple, tbl));
        assert_eq!(new, old, "\"%%w\".%%s internal pattern");
    }
}

// ═══ %r — ordinal ══════════════════════════════════════════════════════

#[test]
fn cmp_r_ordinal() {
    unsafe {
        for &n in &[0i64, 1, 2, 3, 4, 11, 12, 13, 21, 22, 23, 100, 101, 111] {
            let new = fmt2(b"%r\0", &[PrintfArg::Int(n)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%r\0".as_ptr() as _, n as i32));
            assert_eq!(new, old, "%r({n})");
        }
    }
}

// ═══ %% — literal percent ══════════════════════════════════════════════

#[test]
fn cmp_percent() {
    unsafe {
        let new = fmt2(b"100%%\0", &[PrintfArg::None]);
        let old = mprintf_ref(sqlite3_mprintf(b"100%%\0".as_ptr() as _));
        assert_eq!(new, old, "100%%");
    }
}

// ═══ Mixed patterns from codebase ══════════════════════════════════════

#[test]
fn cmp_mixed_d_s() {
    unsafe {
        let msg = b"ORDER BY\0".as_ptr() as *mut c_char;
        let new = fmt2(b"%r %s\0", &[PrintfArg::Int(1), PrintfArg::Str(msg)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%r %s\0".as_ptr() as _, 1i32, msg));
        assert_eq!(new, old, "%r %s");
    }
}

#[test]
fn cmp_mixed_s_d() {
    unsafe {
        let name = b"col\0".as_ptr() as *mut c_char;
        let new = fmt2(
            b"%s=%d (%.2f)\0",
            &[PrintfArg::Str(name), PrintfArg::Int(42), PrintfArg::Double(3.14)],
        );
        let old = mprintf_ref(sqlite3_mprintf(
            b"%s=%d (%.2f)\0".as_ptr() as _,
            name,
            42i32,
            3.14f64,
        ));
        assert_eq!(new, old, "%s=%d (%.2f)");
    }
}

#[test]
fn cmp_date_pattern() {
    unsafe {
        // "%c%04d-%02d-%02d" from date.rs
        let new = fmt2(
            b"%c%04d-%02d-%02d\0",
            &[
                PrintfArg::Char(b'+' as u32),
                PrintfArg::Int(2024),
                PrintfArg::Int(3),
                PrintfArg::Int(15),
            ],
        );
        let old = mprintf_ref(sqlite3_mprintf(
            b"%c%04d-%02d-%02d\0".as_ptr() as _,
            b'+' as i32,
            2024i32,
            3i32,
            15i32,
        ));
        assert_eq!(new, old, "%c%04d-%02d-%02d");
    }
}

// ═══ %f — basic float ══════════════════════════════════════════════════

#[test]
fn cmp_f_basic() {
    unsafe {
        for &v in &[0.0f64, 3.14, -1.5, 100.0] {
            let new = fmt2(b"%.2f\0", &[PrintfArg::Double(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%.2f\0".as_ptr() as _, v));
            assert_eq!(new, old, "%.2f({v})");
        }
    }
}

// ═══ Corner cases: %.*s — raw bytes with dynamic precision ═════════════

#[test]
fn cmp_raw_bytes() {
    unsafe {
        let data = b"hello world\0".as_ptr() as *mut c_char;
        for &n in &[0i32, 1, 3, 5, 11] {
            let new = fmt2(b"%.*s\0", &[PrintfArg::Int(n as i64), PrintfArg::Str(data)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%.*s\0".as_ptr() as _, n, data));
            assert_eq!(new, old, "%.*s(n={n})");
        }
    }
}

#[test]
fn cmp_raw_bytes_with_suffix() {
    unsafe {
        let data = b"hello world\0".as_ptr() as *mut c_char;
        let suffix = b"_end\0".as_ptr() as *mut c_char;
        let new = fmt2(b"%.*s%s\0", &[PrintfArg::Int(5), PrintfArg::Str(data), PrintfArg::Str(suffix)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%.*s%s\0".as_ptr() as _, 5i32, data, suffix));
        assert_eq!(new, old, "%.*s%s");
    }
}

// ═══ Corner cases: internal CREATE pattern ═════════════════════════════

#[test]
fn cmp_create_pattern() {
    // "CREATE %s %.*s" — used by CREATE TABLE/INDEX/VIEW
    unsafe {
        let kind = b"TABLE\0".as_ptr() as *mut c_char;
        let sql = b"t1(a INT, b TEXT)\0".as_ptr() as *mut c_char;
        let len = 17i32;
        let new = fmt2(
            b"CREATE %s %.*s\0",
            &[PrintfArg::Str(kind), PrintfArg::Int(len as i64), PrintfArg::Str(sql)],
        );
        let old = mprintf_ref(sqlite3_mprintf(b"CREATE %s %.*s\0".as_ptr() as _, kind, len, sql));
        assert_eq!(new, old, "CREATE %%s %%.*s");
    }
}

// ═══ Corner cases: %#d alternate form (should be no-op for decimal) ════

#[test]
fn cmp_d_alternate_form() {
    unsafe {
        // %#d — alternate form for decimal is a no-op in SQLite
        for &v in &[0i64, 42, -1] {
            let new = fmt2(b"%#d\0", &[PrintfArg::Int(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%#d\0".as_ptr() as _, v as i32));
            assert_eq!(new, old, "%#d({v})");
        }
    }
}

// ═══ Corner cases: %+d prefix ══════════════════════════════════════════

#[test]
fn cmp_d_plus_prefix() {
    unsafe {
        for &v in &[0i64, 42, -1] {
            let new = fmt2(b"%+d\0", &[PrintfArg::Int(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%+d\0".as_ptr() as _, v as i32));
            assert_eq!(new, old, "%+d({v})");
        }
    }
}

#[test]
fn cmp_d_space_prefix() {
    unsafe {
        for &v in &[0i64, 42, -1] {
            let new = fmt2(b"% d\0", &[PrintfArg::Int(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"% d\0".as_ptr() as _, v as i32));
            assert_eq!(new, old, "% d({v})");
        }
    }
}

// ═══ Corner cases: UPDATE SQL pattern with #%d ═════════════════════════

#[test]
fn cmp_update_rootpage_pattern() {
    // The pattern "rootpage=#%d" where # is a literal character
    unsafe {
        let new = fmt2(b"rootpage=#%d\0", &[PrintfArg::Int(42)]);
        let old = mprintf_ref(sqlite3_mprintf(b"rootpage=#%d\0".as_ptr() as _, 42i32));
        assert_eq!(new, old, "rootpage=#%%d");
    }
}

// ═══ Corner cases: multiple %Q in one string ═══════════════════════════

#[test]
fn cmp_multi_Q() {
    unsafe {
        let a = b"main\0".as_ptr() as *mut c_char;
        let b_str = b"t1\0".as_ptr() as *mut c_char;
        let c_str = b"O'Reilly\0".as_ptr() as *mut c_char;
        let new = fmt2(
            b"UPDATE %Q SET name=%Q, val=%Q\0",
            &[PrintfArg::Str(a), PrintfArg::Str(b_str), PrintfArg::Str(c_str)],
        );
        let old = mprintf_ref(sqlite3_mprintf(
            b"UPDATE %Q SET name=%Q, val=%Q\0".as_ptr() as _,
            a, b_str, c_str,
        ));
        assert_eq!(new, old, "multi %Q");
    }
}

// ═══ Corner cases: %llu — unsigned 64-bit ══════════════════════════════

#[test]
fn cmp_llu_basic() {
    unsafe {
        for &v in &[0u64, 1, u64::MAX] {
            let new = fmt2(b"%llu\0", &[PrintfArg::UInt(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%llu\0".as_ptr() as _, v));
            assert_eq!(new, old, "%llu({v})");
        }
    }
}

// ═══ Corner cases: %o — octal ══════════════════════════════════════════

#[test]
fn cmp_o_basic() {
    unsafe {
        for &v in &[0u64, 7, 8, 255, 0o777] {
            let new = fmt2(b"%o\0", &[PrintfArg::UInt(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%o\0".as_ptr() as _, v as u32));
            assert_eq!(new, old, "%o({v})");
        }
    }
}

// ═══ Corner cases: empty format string ═════════════════════════════════

#[test]
fn cmp_empty_format() {
    unsafe {
        let new = fmt2(b"\0", &[]);
        let old = mprintf_ref(sqlite3_mprintf(b"\0".as_ptr() as _));
        assert_eq!(new, old, "empty format");
    }
}

// ═══ Corner cases: format with no specifiers ═══════════════════════════

#[test]
fn cmp_no_specifiers() {
    unsafe {
        let new = fmt2(b"hello world\0", &[]);
        let old = mprintf_ref(sqlite3_mprintf(b"hello world\0".as_ptr() as _));
        assert_eq!(new, old, "no specifiers");
    }
}

// ═══ Corner cases: %p — pointer ════════════════════════════════════════

#[test]
fn cmp_p_basic() {
    unsafe {
        // %p formats as hex with pointer width
        let v = 0xDEADBEEFu64;
        let new = fmt2(b"%p\0", &[PrintfArg::UInt(v)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%p\0".as_ptr() as _, v as *const c_void));
        assert_eq!(new, old, "%p(0xDEADBEEF)");
    }
}

// ═══ Corner cases: large width values ══════════════════════════════════

#[test]
fn cmp_large_width() {
    unsafe {
        let new = fmt2(b"%20d\0", &[PrintfArg::Int(42)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%20d\0".as_ptr() as _, 42i32));
        assert_eq!(new, old, "%20d(42)");

        let new = fmt2(b"%-20s!\0", &[PrintfArg::Str(b"hi\0".as_ptr() as _)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%-20s!\0".as_ptr() as _, b"hi\0".as_ptr()));
        assert_eq!(new, old, "%-20s(hi)");
    }
}

// ═══ Corner cases: %i (synonym for %d) ═════════════════════════════════

#[test]
fn cmp_i_basic() {
    unsafe {
        for &v in &[0i64, 42, -1] {
            let new = fmt2(b"%i\0", &[PrintfArg::Int(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%i\0".as_ptr() as _, v as i32));
            assert_eq!(new, old, "%i({v})");
        }
    }
}

// ═══ Corner cases: %s with width ═══════════════════════════════════════

#[test]
fn cmp_s_width() {
    unsafe {
        let s = b"hi\0".as_ptr() as *mut c_char;
        let new = fmt2(b"%10s\0", &[PrintfArg::Str(s)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%10s\0".as_ptr() as _, s));
        assert_eq!(new, old, "%10s(hi)");

        let new = fmt2(b"%-10s|\0", &[PrintfArg::Str(s)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%-10s|\0".as_ptr() as _, s));
        assert_eq!(new, old, "%-10s(hi)");
    }
}

// ═══ Float edge values ═════════════════════════════════════════════════
// Tests marked #[ignore] fail due to Rust format!() vs SQLite FpDecode differences.
// They document the exact gaps. Un-ignore as we switch to FpDecode.

#[test]
fn cmp_f_small() {
    unsafe {
        let new = fmt2(b"%.6f\0", &[PrintfArg::Double(0.000001)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%.6f\0".as_ptr() as _, 0.000001f64));
        assert_eq!(new, old, "%.6f(0.000001)");
    }
}

#[test]
fn cmp_f_negative_zero() {
    unsafe {
        let new = fmt2(b"%.1f\0", &[PrintfArg::Double(-0.0)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%.1f\0".as_ptr() as _, -0.0f64));
        assert_eq!(new, old, "%.1f(-0.0)");
    }
}

// ═══ %g — significant digits (SQLite canonical float format) ═══════════

#[test]

fn cmp_g_015() {
    // "%!0.15g" — SQLite's canonical float serialization
    unsafe {
        for &v in &[
            0.0f64, 1.0, -1.0, 0.5, 1.0/3.0,
            3.14159265358979323846,
            1.23456789012345678,
            1e10, 1e-10, 1e100, 1e-100,
            f64::MAX, f64::MIN_POSITIVE,
        ] {
            let new = fmt2(b"%!0.15g\0", &[PrintfArg::Double(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%!0.15g\0".as_ptr() as _, v));
            assert_eq!(new, old, "%!0.15g({v})");
        }
    }
}

#[test]

fn cmp_g_15_variant() {
    unsafe {
        for &v in &[1.0f64, 3.14159265358979, 1.0/3.0] {
            let new = fmt2(b"%!.15g\0", &[PrintfArg::Double(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%!.15g\0".as_ptr() as _, v));
            assert_eq!(new, old, "%!.15g({v})");
        }
    }
}

#[test]

fn cmp_g_16() {
    unsafe {
        for &v in &[1.0f64, 3.14159265358979, 0.1 + 0.2, 1e15, 1e-15] {
            let new = fmt2(b"%.16g\0", &[PrintfArg::Double(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%.16g\0".as_ptr() as _, v));
            assert_eq!(new, old, "%.16g({v})");
        }
    }
}

// ═══ %e — scientific notation ══════════════════════════════════════════

#[test]

fn cmp_e_basic() {
    unsafe {
        for &v in &[0.0f64, 1.0, -1.0, 3.14, 1e10, 1e-10, 1e100] {
            let new = fmt2(b"%.6e\0", &[PrintfArg::Double(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%.6e\0".as_ptr() as _, v));
            assert_eq!(new, old, "%.6e({v})");
        }
    }
}

#[test]

fn cmp_e_020() {
    unsafe {
        for &v in &[1.0f64, 3.14159265358979, f64::MAX] {
            let new = fmt2(b"%!0.20e\0", &[PrintfArg::Double(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%!0.20e\0".as_ptr() as _, v));
            assert_eq!(new, old, "%!0.20e({v})");
        }
    }
}

// ═══ %!.*f — dynamic precision with trailing-zero stripping ════════════

#[test]

fn cmp_f_dynamic_prec_strip() {
    unsafe {
        for &(prec, val) in &[
            (0i32, 3.14159f64),
            (3, 3.14159),
            (6, 1.0),
            (2, 0.0),
            (3, -1.5),
            (10, 1.0/3.0),
        ] {
            let new = fmt2(
                b"%!.*f\0",
                &[PrintfArg::Int(prec as i64), PrintfArg::Double(val)],
            );
            let old = mprintf_ref(sqlite3_mprintf(b"%!.*f\0".as_ptr() as _, prec, val));
            assert_eq!(new, old, "%!.*f(prec={prec}, val={val})");
        }
    }
}

// ═══ %.3f — fixed precision (common in codebase) ═══════════════════════

#[test]
fn cmp_f_fixed_3() {
    unsafe {
        for &v in &[0.0f64, 3.14159, -1.5, 1000.0, 0.001, 0.9999] {
            let new = fmt2(b"%.3f\0", &[PrintfArg::Double(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%.3f\0".as_ptr() as _, v));
            assert_eq!(new, old, "%.3f({v})");
        }
    }
}

// ═══ %06.3f — zero-padded fixed float ══════════════════════════════════

#[test]

fn cmp_f_zeropad() {
    unsafe {
        for &v in &[3.14159f64, 0.0, -1.5, 99.999] {
            let new = fmt2(b"%06.3f\0", &[PrintfArg::Double(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%06.3f\0".as_ptr() as _, v));
            assert_eq!(new, old, "%06.3f({v})");
        }
    }
}

// ═══ %g — plain g format ═══════════════════════════════════════════════

#[test]

fn cmp_g_plain() {
    unsafe {
        for &v in &[0.0f64, 1.0, 0.5, 100.0, 0.00001, 1e10] {
            let new = fmt2(b"%g\0", &[PrintfArg::Double(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%g\0".as_ptr() as _, v));
            assert_eq!(new, old, "%g({v})");
        }
    }
}

// ═══ %c%g,%g — geopolygon coord pattern ═══════════════════════════════

#[test]

fn cmp_geopolygon_pattern() {
    unsafe {
        let new = fmt2(
            b"%c%g,%g\0",
            &[PrintfArg::Char(b'[' as u32), PrintfArg::Double(1.5), PrintfArg::Double(2.5)],
        );
        let old = mprintf_ref(sqlite3_mprintf(
            b"%c%g,%g\0".as_ptr() as _, b'[' as i32, 1.5f64, 2.5f64,
        ));
        assert_eq!(new, old, "%c%g,%g");
    }
}

// ═══ datetime pattern with float seconds ═══════════════════════════════

#[test]

fn cmp_datetime_full() {
    unsafe {
        let new = fmt2(
            b"%c%04d-%02d-%02d %02d:%02d:%06.3f\0",
            &[
                PrintfArg::Char(b'+' as u32),
                PrintfArg::Int(2024), PrintfArg::Int(3), PrintfArg::Int(15),
                PrintfArg::Int(10), PrintfArg::Int(30),
                PrintfArg::Double(5.123),
            ],
        );
        let old = mprintf_ref(sqlite3_mprintf(
            b"%c%04d-%02d-%02d %02d:%02d:%06.3f\0".as_ptr() as _,
            b'+' as i32, 2024i32, 3i32, 15i32, 10i32, 30i32, 5.123f64,
        ));
        assert_eq!(new, old, "datetime full");
    }
}

// ═══ Infinity and NaN ══════════════════════════════════════════════════

#[test]

fn cmp_f_infinity_nan() {
    unsafe {
        let new = fmt2(b"%.2f\0", &[PrintfArg::Double(f64::INFINITY)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%.2f\0".as_ptr() as _, f64::INFINITY));
        assert_eq!(new, old, "%.2f(Inf)");

        let new = fmt2(b"%.2f\0", &[PrintfArg::Double(f64::NEG_INFINITY)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%.2f\0".as_ptr() as _, f64::NEG_INFINITY));
        assert_eq!(new, old, "%.2f(-Inf)");

        let new = fmt2(b"%.2f\0", &[PrintfArg::Double(f64::NAN)]);
        let old = mprintf_ref(sqlite3_mprintf(b"%.2f\0".as_ptr() as _, f64::NAN));
        assert_eq!(new, old, "%.2f(NaN)");
    }
}

// ═══ %f default precision (6 decimal places) ═══════════════════════════

#[test]
fn cmp_f_default_precision() {
    unsafe {
        for &v in &[0.0f64, 1.0, 3.14159265, -42.1] {
            let new = fmt2(b"%f\0", &[PrintfArg::Double(v)]);
            let old = mprintf_ref(sqlite3_mprintf(b"%f\0".as_ptr() as _, v));
            assert_eq!(new, old, "%f({v})");
        }
    }
}
