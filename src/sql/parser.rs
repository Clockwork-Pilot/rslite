//! Recursive-descent / Pratt parser for SQLite SQL.
//!
//! Entry point: [`parse_sql`] — takes a SQL string, returns `Vec<Stmt>`.

use super::ast::*;
use super::lexer::{Token, TokenKind, tokenize};

// ── Error ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub offset:  usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for ParseError {}

// ── Parser ────────────────────────────────────────────────────────────────────

struct Parser {
    tokens: Vec<Token>,
    pos:    usize,
}

// ── Pratt precedences (higher = tighter binding) ──────────────────────────────
const PREC_OR:     u8 = 1;
const PREC_AND:    u8 = 2;
const PREC_CMP:    u8 = 4;   // =, <>, !=, IS, IN, LIKE, BETWEEN …
const PREC_REL:    u8 = 5;   // <, <=, >, >=
const PREC_BITOR:  u8 = 6;   // |
const PREC_BITAND: u8 = 7;   // &
const PREC_SHIFT:  u8 = 8;   // <<, >>
const PREC_ADD:    u8 = 9;   // +, -
const PREC_MUL:    u8 = 10;  // *, /, %
const PREC_CONCAT: u8 = 11;  // ||

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn offset(&self) -> usize {
        self.tokens.get(self.pos).map_or(0, |t| t.offset)
    }

    fn peek(&self) -> &TokenKind {
        self.tokens.get(self.pos).map_or(&TokenKind::Eof, |t| &t.kind)
    }

    fn peek_at(&self, n: usize) -> &TokenKind {
        self.tokens.get(self.pos + n).map_or(&TokenKind::Eof, |t| &t.kind)
    }

    fn advance(&mut self) -> TokenKind {
        if self.pos < self.tokens.len() {
            let k = self.tokens[self.pos].kind.clone();
            self.pos += 1;
            k
        } else {
            TokenKind::Eof
        }
    }

    fn eat(&mut self, kind: &TokenKind) -> bool {
        if self.peek() == kind { self.pos += 1; true } else { false }
    }

    fn expect(&mut self, kind: &TokenKind) -> Result<(), ParseError> {
        if self.eat(kind) { Ok(()) }
        else { Err(self.err(format!("expected {:?}, found {:?}", kind, self.peek()))) }
    }

    fn err(&self, msg: impl Into<String>) -> ParseError {
        ParseError { message: msg.into(), offset: self.offset() }
    }

    /// Consume an identifier or any keyword that SQLite allows as a bare name.
    fn expect_name(&mut self) -> Result<String, ParseError> {
        match self.peek().clone() {
            TokenKind::Identifier(s) => { self.pos += 1; Ok(s) }
            TokenKind::String(s)      => { self.pos += 1; Ok(s) }
            other => {
                if let Some(s) = kw_as_name(&other) { self.pos += 1; Ok(s) }
                else { Err(self.err(format!("expected name, found {:?}", other))) }
            }
        }
    }

    /// Try to consume an identifier/keyword-name; return None without consuming if it isn't one.
    fn try_name(&mut self) -> Option<String> {
        match self.peek().clone() {
            TokenKind::Identifier(s) => { self.pos += 1; Some(s) }
            other => kw_as_name(&other).inspect(|_s| { self.pos += 1; }),
        }
    }

    // ── Top-level ─────────────────────────────────────────────────────────────

    fn parse_stmts(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        loop {
            while self.eat(&TokenKind::Semi) {}
            if matches!(self.peek(), TokenKind::Eof) { break; }
            stmts.push(self.parse_stmt()?);
            if !self.eat(&TokenKind::Semi)
                && !matches!(self.peek(), TokenKind::Eof) {
                    return Err(self.err(format!("expected ';' or end of input, found {:?}", self.peek())));
                }
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek().clone() {
            TokenKind::Explain => {
                self.pos += 1;
                if self.eat(&TokenKind::Query) {
                    self.expect(&TokenKind::Plan)?;
                    Ok(Stmt::ExplainQueryPlan(Box::new(self.parse_stmt()?)))
                } else {
                    Ok(Stmt::Explain(Box::new(self.parse_stmt()?)))
                }
            }
            TokenKind::Select | TokenKind::Values | TokenKind::With => {
                Ok(Stmt::Select(self.parse_select_stmt()?))
            }
            TokenKind::Insert | TokenKind::Replace => {
                Ok(Stmt::Insert(self.parse_insert_stmt()?))
            }
            TokenKind::Update => Ok(Stmt::Update(self.parse_update_stmt()?)),
            TokenKind::Delete => Ok(Stmt::Delete(self.parse_delete_stmt()?)),
            TokenKind::Create => self.parse_create(),
            TokenKind::Drop   => self.parse_drop(),
            TokenKind::Begin  => self.parse_begin(),
            TokenKind::Commit | TokenKind::End => {
                self.pos += 1; self.eat(&TokenKind::Transaction); Ok(Stmt::Commit)
            }
            TokenKind::Rollback => {
                self.pos += 1; self.eat(&TokenKind::Transaction);
                if self.eat(&TokenKind::To) {
                    self.eat(&TokenKind::Savepoint);
                    let name = self.expect_name()?;
                    Ok(Stmt::Release(name)) // reuse Release as rollback-to
                } else {
                    Ok(Stmt::Rollback)
                }
            }
            TokenKind::Savepoint => {
                self.pos += 1;
                Ok(Stmt::Savepoint(self.expect_name()?))
            }
            TokenKind::Release => {
                self.pos += 1; self.eat(&TokenKind::Savepoint);
                Ok(Stmt::Release(self.expect_name()?))
            }
            TokenKind::Pragma  => self.parse_pragma(),
            TokenKind::Attach  => self.parse_attach(),
            TokenKind::Detach  => self.parse_detach(),
            TokenKind::Reindex => {
                self.pos += 1;
                Ok(Stmt::Reindex(self.try_name()))
            }
            TokenKind::Vacuum => {
                self.pos += 1;
                Ok(Stmt::Vacuum(self.try_name()))
            }
            TokenKind::Analyze => {
                self.pos += 1;
                Ok(Stmt::Analyze(self.try_name()))
            }
            _ => Err(self.err(format!("unexpected token {:?}", self.peek()))),
        }
    }

    // ── SELECT ────────────────────────────────────────────────────────────────

    fn parse_select_stmt(&mut self) -> Result<SelectStmt, ParseError> {
        let with = if self.eat(&TokenKind::With) { Some(self.parse_with_clause()?) } else { None };
        let core = self.parse_select_core()?;
        let mut compounds = Vec::new();
        loop {
            let op = match self.peek() {
                TokenKind::Union => {
                    self.pos += 1;
                    if self.eat(&TokenKind::All) { CompoundOp::UnionAll } else { CompoundOp::Union }
                }
                TokenKind::Intersect => { self.pos += 1; CompoundOp::Intersect }
                TokenKind::Except    => { self.pos += 1; CompoundOp::Except }
                _ => break,
            };
            compounds.push((op, self.parse_select_core()?));
        }
        let order_by = if self.eat(&TokenKind::Order) {
            self.expect(&TokenKind::By)?;
            self.parse_ordering_terms()?
        } else { Vec::new() };
        // Check if ORDER BY was parsed but there's a UNION/INTERSECT/EXCEPT after it (error case)
        if !order_by.is_empty() {
            let msg = match self.peek() {
                TokenKind::Union => {
                    if matches!(self.peek_at(1), TokenKind::All) {
                        "ORDER BY clause should come after UNION ALL not before"
                    } else {
                        "ORDER BY clause should come after UNION not before"
                    }
                }
                TokenKind::Intersect => "ORDER BY clause should come after INTERSECT not before",
                TokenKind::Except => "ORDER BY clause should come after EXCEPT not before",
                _ => "",
            };
            if !msg.is_empty() {
                return Err(self.err(msg));
            }
        }
        let limit = if self.eat(&TokenKind::Limit) { Some(self.parse_limit()?) } else { None };
        // Check if LIMIT was parsed but there's a UNION/INTERSECT/EXCEPT after it (error case)
        if limit.is_some() {
            let msg = match self.peek() {
                TokenKind::Union => {
                    if matches!(self.peek_at(1), TokenKind::All) {
                        "LIMIT clause should come after UNION ALL not before"
                    } else {
                        "LIMIT clause should come after UNION not before"
                    }
                }
                TokenKind::Intersect => "LIMIT clause should come after INTERSECT not before",
                TokenKind::Except => "LIMIT clause should come after EXCEPT not before",
                _ => "",
            };
            if !msg.is_empty() {
                return Err(self.err(msg));
            }
        }
        Ok(SelectStmt { with, core, compounds, order_by, limit })
    }

    fn parse_select_core(&mut self) -> Result<SelectCore, ParseError> {
        // VALUES clause
        if self.eat(&TokenKind::Values) {
            // Treat VALUES(...) as SELECT with literal columns for now
            self.expect(&TokenKind::LParen)?;
            let mut exprs = Vec::new();
            loop {
                exprs.push(ResultColumn::Expr { expr: self.parse_expr(0)?, alias: None });
                if !self.eat(&TokenKind::Comma) { break; }
            }
            self.expect(&TokenKind::RParen)?;
            return Ok(SelectCore { distinct: false, columns: exprs, from: None, where_: None, group_by: Vec::new(), having: None });
        }
        self.expect(&TokenKind::Select)?;
        let distinct = if self.eat(&TokenKind::Distinct) { true }
                       else { self.eat(&TokenKind::All); false };
        let columns = self.parse_result_columns()?;
        let from = if self.eat(&TokenKind::From) { Some(self.parse_from_clause()?) } else { None };
        let where_ = if self.eat(&TokenKind::Where) { Some(self.parse_expr(0)?) } else { None };
        let group_by = if self.eat(&TokenKind::Group) {
            self.expect(&TokenKind::By)?;
            self.parse_expr_list()?
        } else { Vec::new() };
        let having = if self.eat(&TokenKind::Having) { Some(self.parse_expr(0)?) } else { None };
        // WINDOW clause not yet implemented; skip it
        if matches!(self.peek(), TokenKind::Window) { self.pos += 1; self.expect_name()?; self.expect(&TokenKind::As)?; self.expect(&TokenKind::LParen)?; self.skip_balanced_paren()?; }
        Ok(SelectCore { distinct, columns, from, where_, group_by, having })
    }

    fn parse_result_columns(&mut self) -> Result<Vec<ResultColumn>, ParseError> {
        let mut cols = Vec::new();
        loop {
            cols.push(self.parse_result_column()?);
            if !self.eat(&TokenKind::Comma) { break; }
        }
        Ok(cols)
    }

    fn parse_result_column(&mut self) -> Result<ResultColumn, ParseError> {
        // table.* or just *
        if self.eat(&TokenKind::Star) {
            return Ok(ResultColumn::Star);
        }
        // name.*
        if let TokenKind::Identifier(_) = self.peek() {
            if matches!(self.peek_at(1), TokenKind::Dot) && matches!(self.peek_at(2), TokenKind::Star) {
                let name = self.expect_name()?;
                self.pos += 2; // skip . and *
                return Ok(ResultColumn::TableStar(name));
            }
        }
        let expr = self.parse_expr(0)?;
        let alias = self.parse_alias()?;
        Ok(ResultColumn::Expr { expr, alias })
    }

    fn parse_alias(&mut self) -> Result<Option<String>, ParseError> {
        if self.eat(&TokenKind::As) {
            Ok(Some(self.expect_name()?))
        } else {
            // Implicit alias: bare identifier not followed by a operator
            match self.peek() {
                TokenKind::Identifier(_) => Ok(Some(self.expect_name()?)),
                _ => Ok(None),
            }
        }
    }

    fn parse_from_clause(&mut self) -> Result<FromClause, ParseError> {
        let mut items = Vec::new();
        let mut joins = Vec::new();
        items.push(self.parse_table_or_subquery()?);

        // Additional comma-joined tables or explicit JOINs
        loop {
            if self.eat(&TokenKind::Comma) {
                // Comma-separated table (CROSS JOIN)
                items.push(self.parse_table_or_subquery()?);
                // Check if there's an ON or USING after comma-separated table
                // If so, convert it to an implicit INNER JOIN
                if matches!(self.peek(), TokenKind::On | TokenKind::Using) {
                    let constraint = if self.eat(&TokenKind::On) {
                        Some(JoinConstraint::On(self.parse_expr(0)?))
                    } else if self.eat(&TokenKind::Using) {
                        self.expect(&TokenKind::LParen)?;
                        let cols = self.parse_name_list()?;
                        self.expect(&TokenKind::RParen)?;
                        Some(JoinConstraint::Using(cols))
                    } else {
                        None
                    };

                    // Pop the last item and convert it to a JOIN
                    if let Some(table_item) = items.pop() {
                        let table = match table_item {
                            TableOrSubquery::Table(tref) => TableOrSubquery::Table(tref),
                            TableOrSubquery::Subquery { select, alias } => {
                                TableOrSubquery::Subquery { select, alias }
                            }
                        };
                        joins.push(JoinClause {
                            join_type: JoinType::Inner,
                            table,
                            constraint,
                        });
                    }
                }
            } else if let Some(jc) = self.try_parse_join()? {
                // Explicit JOIN clause
                joins.push(jc);
            } else {
                break;
            }
        }

        Ok(FromClause { items, joins })
    }

    fn parse_table_or_subquery(&mut self) -> Result<TableOrSubquery, ParseError> {
        if self.eat(&TokenKind::LParen) {
            if matches!(self.peek(), TokenKind::Select | TokenKind::With | TokenKind::Values) {
                let sel = self.parse_select_stmt()?;
                self.expect(&TokenKind::RParen)?;
                let alias = self.parse_alias()?;
                return Ok(TableOrSubquery::Subquery { select: Box::new(sel), alias });
            }
            // Parenthesised table list — treat like a single table for now
            let inner = self.parse_table_or_subquery()?;
            self.expect(&TokenKind::RParen)?;
            return Ok(inner);
        }
        let first = self.expect_name()?;
        let (schema, name) = if self.eat(&TokenKind::Dot) {
            (Some(first), self.expect_name()?)
        } else {
            (None, first)
        };
        // Skip indexed-by / not indexed
        if matches!(self.peek(), TokenKind::Indexed) {
            self.pos += 1; self.expect(&TokenKind::By)?; self.expect_name()?;
        } else if matches!(self.peek(), TokenKind::Not) && matches!(self.peek_at(1), TokenKind::Indexed) {
            self.pos += 2;
        }
        let alias = if self.eat(&TokenKind::As) {
            Some(self.expect_name()?)
        } else {
            // implicit alias: bare name that isn't a keyword starting a new clause
            if is_table_alias_start(self.peek()) {
                self.try_name()
            } else {
                None
            }
        };
        Ok(TableOrSubquery::Table(TableRef { schema, name, alias }))
    }

    fn try_parse_join(&mut self) -> Result<Option<JoinClause>, ParseError> {
        let join_type = match self.peek() {
            TokenKind::Join                   => { self.pos += 1; JoinType::Inner }
            TokenKind::Inner                  => { self.pos += 1; self.eat(&TokenKind::Join); JoinType::Inner }
            TokenKind::Cross                  => { self.pos += 1; self.eat(&TokenKind::Join); JoinType::Cross }
            TokenKind::Natural                => {
                self.pos += 1;
                match self.peek() {
                    TokenKind::Left  => { self.pos += 1; self.eat(&TokenKind::Outer); self.eat(&TokenKind::Join); JoinType::Natural }
                    TokenKind::Right => { self.pos += 1; self.eat(&TokenKind::Outer); self.eat(&TokenKind::Join); JoinType::Natural }
                    TokenKind::Full  => { self.pos += 1; self.eat(&TokenKind::Outer); self.eat(&TokenKind::Join); JoinType::Natural }
                    _                => { self.eat(&TokenKind::Join); JoinType::Natural }
                }
            }
            TokenKind::Left  => {
                self.pos += 1; self.eat(&TokenKind::Outer); self.eat(&TokenKind::Join);
                JoinType::Left
            }
            TokenKind::Right => {
                self.pos += 1; self.eat(&TokenKind::Outer); self.eat(&TokenKind::Join);
                JoinType::Right
            }
            TokenKind::Full  => {
                self.pos += 1; self.eat(&TokenKind::Outer); self.eat(&TokenKind::Join);
                JoinType::Full
            }
            _ => return Ok(None),
        };
        let table = self.parse_table_or_subquery()?;
        let constraint = if self.eat(&TokenKind::On) {
            Some(JoinConstraint::On(self.parse_expr(0)?))
        } else if self.eat(&TokenKind::Using) {
            self.expect(&TokenKind::LParen)?;
            let cols = self.parse_name_list()?;
            self.expect(&TokenKind::RParen)?;
            Some(JoinConstraint::Using(cols))
        } else {
            None
        };
        Ok(Some(JoinClause { join_type, table, constraint }))
    }

    fn parse_ordering_terms(&mut self) -> Result<Vec<OrderingTerm>, ParseError> {
        let mut terms = Vec::new();
        loop {
            let expr = self.parse_expr(0)?;
            let asc = if self.eat(&TokenKind::Desc) { false }
                      else { self.eat(&TokenKind::Asc); true };
            let nulls_first = if self.eat(&TokenKind::Nulls) {
                if self.eat(&TokenKind::First) { Some(true) }
                else { self.expect(&TokenKind::Last)?; Some(false) }
            } else { None };
            terms.push(OrderingTerm { expr, asc, nulls_first });
            if !self.eat(&TokenKind::Comma) { break; }
        }
        Ok(terms)
    }

    fn parse_limit(&mut self) -> Result<LimitClause, ParseError> {
        let limit = self.parse_expr(0)?;
        let offset = if self.eat(&TokenKind::Offset) {
            Some(self.parse_expr(0)?)
        } else if self.eat(&TokenKind::Comma) {
            // old-style LIMIT offset, count — comma form: first is offset
            Some(self.parse_expr(0)?)
        } else {
            None
        };
        Ok(LimitClause { limit, offset })
    }

    // ── WITH ──────────────────────────────────────────────────────────────────

    fn parse_with_clause(&mut self) -> Result<WithClause, ParseError> {
        let recursive = self.eat(&TokenKind::Recursive);
        let mut ctes = Vec::new();
        loop {
            let name = self.expect_name()?;
            let columns = if self.eat(&TokenKind::LParen) {
                let cols = self.parse_name_list()?;
                self.expect(&TokenKind::RParen)?;
                cols
            } else { Vec::new() };
            self.expect(&TokenKind::As)?;
            let materialized = if self.eat(&TokenKind::Not) {
                self.expect(&TokenKind::Materialized)?; Some(false)
            } else if self.eat(&TokenKind::Materialized) {
                Some(true)
            } else { None };
            self.expect(&TokenKind::LParen)?;
            let body = self.parse_select_stmt()?;
            self.expect(&TokenKind::RParen)?;
            ctes.push(Cte { name, columns, body: Box::new(body), materialized });
            if !self.eat(&TokenKind::Comma) { break; }
        }
        Ok(WithClause { recursive, ctes })
    }

    // ── INSERT ────────────────────────────────────────────────────────────────

    fn parse_insert_stmt(&mut self) -> Result<InsertStmt, ParseError> {
        let with = if matches!(self.peek(), TokenKind::With) {
            self.pos += 1; Some(self.parse_with_clause()?)
        } else { None };
        let or = if self.eat(&TokenKind::Replace) {
            Some(ConflictAction::Replace)
        } else {
            self.expect(&TokenKind::Insert)?;
            if self.eat(&TokenKind::Or) { Some(self.parse_conflict_action()?) } else { None }
        };
        self.expect(&TokenKind::Into)?;
        let (schema, table) = self.parse_schema_name()?;
        let alias = if self.eat(&TokenKind::As) { Some(self.expect_name()?) } else { None };
        let columns = if self.eat(&TokenKind::LParen) {
            let cols = self.parse_name_list()?;
            self.expect(&TokenKind::RParen)?;
            cols
        } else { Vec::new() };
        let source = if self.eat(&TokenKind::Default) {
            self.expect(&TokenKind::Values)?;
            InsertSource::DefaultValues
        } else if matches!(self.peek(), TokenKind::Select | TokenKind::With) {
            InsertSource::Select(Box::new(self.parse_select_stmt()?))
        } else {
            self.expect(&TokenKind::Values)?;
            let mut rows = Vec::new();
            loop {
                self.expect(&TokenKind::LParen)?;
                rows.push(self.parse_expr_list()?);
                self.expect(&TokenKind::RParen)?;
                if !self.eat(&TokenKind::Comma) { break; }
            }
            InsertSource::Values(rows)
        };
        let returning = self.parse_returning()?;
        Ok(InsertStmt { with, or, schema, table, alias, columns, source, returning })
    }

    // ── UPDATE ────────────────────────────────────────────────────────────────

    fn parse_update_stmt(&mut self) -> Result<UpdateStmt, ParseError> {
        let with = if matches!(self.peek(), TokenKind::With) {
            self.pos += 1; Some(self.parse_with_clause()?)
        } else { None };
        self.expect(&TokenKind::Update)?;
        let or = if self.eat(&TokenKind::Or) { Some(self.parse_conflict_action()?) } else { None };
        let (schema, table) = self.parse_schema_name()?;
        let alias = if self.eat(&TokenKind::As) { Some(self.expect_name()?) } else { None };
        self.expect(&TokenKind::Set)?;
        let assignments = self.parse_assignments()?;
        let from = if self.eat(&TokenKind::From) { Some(self.parse_from_clause()?) } else { None };
        let where_ = if self.eat(&TokenKind::Where) { Some(self.parse_expr(0)?) } else { None };
        let returning = self.parse_returning()?;
        Ok(UpdateStmt { with, or, schema, table, alias, assignments, from, where_, returning })
    }

    fn parse_assignments(&mut self) -> Result<Vec<Assignment>, ParseError> {
        let mut assigns = Vec::new();
        loop {
            let columns = if self.eat(&TokenKind::LParen) {
                let cols = self.parse_name_list()?;
                self.expect(&TokenKind::RParen)?;
                cols
            } else {
                vec![self.expect_name()?]
            };
            self.expect(&TokenKind::Eq)?;
            let value = self.parse_expr(0)?;
            assigns.push(Assignment { columns, value });
            if !self.eat(&TokenKind::Comma) { break; }
        }
        Ok(assigns)
    }

    // ── DELETE ────────────────────────────────────────────────────────────────

    fn parse_delete_stmt(&mut self) -> Result<DeleteStmt, ParseError> {
        let with = if matches!(self.peek(), TokenKind::With) {
            self.pos += 1; Some(self.parse_with_clause()?)
        } else { None };
        self.expect(&TokenKind::Delete)?;
        self.expect(&TokenKind::From)?;
        let (schema, table) = self.parse_schema_name()?;
        let alias = if self.eat(&TokenKind::As) { Some(self.expect_name()?) } else { None };
        let where_ = if self.eat(&TokenKind::Where) { Some(self.parse_expr(0)?) } else { None };
        let returning = self.parse_returning()?;
        Ok(DeleteStmt { with, schema, table, alias, where_, returning })
    }

    // ── CREATE ────────────────────────────────────────────────────────────────

    fn parse_create(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Create)?;
        let temporary = self.eat(&TokenKind::Temp) || self.eat(&TokenKind::Temporary);
        match self.peek().clone() {
            TokenKind::Table => { self.pos += 1; self.parse_create_table(temporary) }
            TokenKind::Index => { self.pos += 1; self.parse_create_index(false) }
            TokenKind::Unique => {
                self.pos += 1; self.expect(&TokenKind::Index)?;
                self.parse_create_index(true)
            }
            TokenKind::View | TokenKind::Trigger => {
                // Skip for now — skip to end of statement
                while !matches!(self.peek(), TokenKind::Semi | TokenKind::Eof) { self.pos += 1; }
                Ok(Stmt::Empty)
            }
            other => Err(self.err(format!("expected TABLE or INDEX after CREATE, found {:?}", other))),
        }
    }

    fn parse_create_table(&mut self, temporary: bool) -> Result<Stmt, ParseError> {
        let if_not_exists = self.eat_if_not_exists();
        let (schema, name) = self.parse_schema_name()?;
        let body = if self.eat(&TokenKind::As) {
            CreateTableBody::As(Box::new(self.parse_select_stmt()?))
        } else {
            self.expect(&TokenKind::LParen)?;
            let mut columns = Vec::new();
            let mut table_constraints = Vec::new();
            loop {
                if self.is_table_constraint_start() {
                    table_constraints.push(self.parse_table_constraint()?);
                } else {
                    columns.push(self.parse_column_def()?);
                }
                if !self.eat(&TokenKind::Comma) { break; }
                // Allow trailing comma before table constraints or closing paren
                if matches!(self.peek(), TokenKind::RParen) { break; }
            }
            self.expect(&TokenKind::RParen)?;
            CreateTableBody::Columns { columns, table_constraints }
        };
        let without_rowid = if matches!(self.peek(), TokenKind::Without) {
            self.pos += 1;
            let name = self.expect_name()?;
            name.eq_ignore_ascii_case("rowid")
        } else { false };
        let strict = if let TokenKind::Identifier(s) = self.peek() {
            if s.eq_ignore_ascii_case("STRICT") { self.pos += 1; true } else { false }
        } else { false };
        // Allow WITHOUT ROWID, STRICT in either order
        let _ = strict; let _ = without_rowid;
        Ok(Stmt::CreateTable(CreateTableStmt { temporary, if_not_exists, schema, name, body, without_rowid, strict }))
    }

    fn is_table_constraint_start(&self) -> bool {
        matches!(self.peek(), TokenKind::Primary | TokenKind::Unique | TokenKind::Check | TokenKind::Foreign)
            || (matches!(self.peek(), TokenKind::Constraint) )
    }

    fn parse_column_def(&mut self) -> Result<ColumnDef, ParseError> {
        let name = self.expect_name()?;
        let type_name = self.try_parse_type_name();
        let mut constraints = Vec::new();
        while let Some(c) = self.try_parse_column_constraint()? {
                constraints.push(c);
            } 
        Ok(ColumnDef { name, type_name, constraints })
    }

    fn try_parse_type_name(&mut self) -> Option<TypeName> {
        let name = match self.peek() {
            TokenKind::Identifier(_) => self.try_name()?,
            other => kw_as_name(other).inspect(|_s| { self.pos += 1; })?,
        };
        // Accumulate additional name parts (e.g. "DOUBLE PRECISION")
        let mut full = name;
        loop {
            match self.peek() {
                TokenKind::Comma | TokenKind::RParen | TokenKind::LParen
                | TokenKind::Constraint | TokenKind::Primary | TokenKind::Not
                | TokenKind::Unique | TokenKind::Check | TokenKind::Default
                | TokenKind::Collate | TokenKind::References | TokenKind::Generated
                | TokenKind::As | TokenKind::Eof | TokenKind::Semi => break,
                _ => {
                    if let Some(part) = self.try_name() { full.push(' '); full.push_str(&part); }
                    else { break; }
                }
            }
        }
        let mut args = Vec::new();
        if self.eat(&TokenKind::LParen) {
            if let TokenKind::Integer(n) = self.peek().clone() { self.pos += 1; args.push(n); }
            if self.eat(&TokenKind::Comma) {
                if let TokenKind::Integer(n) = self.peek().clone() { self.pos += 1; args.push(n); }
            }
            let _ = self.eat(&TokenKind::RParen);
        }
        Some(TypeName { name: full, args })
    }

    fn try_parse_column_constraint(&mut self) -> Result<Option<ColumnConstraint>, ParseError> {
        // Skip CONSTRAINT name prefix
        if self.eat(&TokenKind::Constraint) { self.expect_name()?; }
        match self.peek().clone() {
            TokenKind::Primary => {
                self.pos += 1; self.expect(&TokenKind::Key)?;
                let asc = if self.eat(&TokenKind::Desc) { false } else { self.eat(&TokenKind::Asc); true };
                let conflict = self.try_parse_on_conflict()?;
                let autoincrement = self.eat(&TokenKind::Autoincrement);
                Ok(Some(ColumnConstraint::PrimaryKey { asc, conflict, autoincrement }))
            }
            TokenKind::Not => {
                self.pos += 1; self.expect(&TokenKind::Null)?;
                Ok(Some(ColumnConstraint::NotNull { conflict: self.try_parse_on_conflict()? }))
            }
            TokenKind::Unique => {
                self.pos += 1;
                Ok(Some(ColumnConstraint::Unique { conflict: self.try_parse_on_conflict()? }))
            }
            TokenKind::Check => {
                self.pos += 1; self.expect(&TokenKind::LParen)?;
                let e = self.parse_expr(0)?;
                self.expect(&TokenKind::RParen)?;
                Ok(Some(ColumnConstraint::Check(e)))
            }
            TokenKind::Default => {
                self.pos += 1;
                let e = if self.eat(&TokenKind::LParen) {
                    let e = self.parse_expr(0)?; self.expect(&TokenKind::RParen)?; e
                } else {
                    self.parse_literal_expr()?
                };
                Ok(Some(ColumnConstraint::Default(e)))
            }
            TokenKind::Collate => {
                self.pos += 1;
                Ok(Some(ColumnConstraint::Collate(self.expect_name()?)))
            }
            TokenKind::References => {
                self.pos += 1;
                Ok(Some(ColumnConstraint::References(self.parse_fk_clause()?)))
            }
            TokenKind::Generated => {
                self.pos += 1; self.eat(&TokenKind::Always);
                self.expect(&TokenKind::As)?;
                self.expect(&TokenKind::LParen)?;
                let e = self.parse_expr(0)?;
                self.expect(&TokenKind::RParen)?;
                let stored = if let TokenKind::Identifier(s) = self.peek() {
                    let stored = s.eq_ignore_ascii_case("STORED");
                    if stored || s.eq_ignore_ascii_case("VIRTUAL") { self.pos += 1; stored } else { false }
                } else { false };
                Ok(Some(ColumnConstraint::Generated { expr: e, stored }))
            }
            _ => Ok(None),
        }
    }

    fn parse_table_constraint(&mut self) -> Result<TableConstraint, ParseError> {
        let name = if self.eat(&TokenKind::Constraint) { Some(self.expect_name()?) } else { None };
        match self.peek().clone() {
            TokenKind::Primary => {
                self.pos += 1; self.expect(&TokenKind::Key)?;
                self.expect(&TokenKind::LParen)?;
                let columns = self.parse_indexed_columns()?;
                self.expect(&TokenKind::RParen)?;
                let conflict = self.try_parse_on_conflict()?;
                Ok(TableConstraint::PrimaryKey { name, columns, conflict })
            }
            TokenKind::Unique => {
                self.pos += 1;
                self.expect(&TokenKind::LParen)?;
                let columns = self.parse_indexed_columns()?;
                self.expect(&TokenKind::RParen)?;
                let conflict = self.try_parse_on_conflict()?;
                Ok(TableConstraint::Unique { name, columns, conflict })
            }
            TokenKind::Check => {
                self.pos += 1; self.expect(&TokenKind::LParen)?;
                let expr = self.parse_expr(0)?;
                self.expect(&TokenKind::RParen)?;
                Ok(TableConstraint::Check { name, expr })
            }
            TokenKind::Foreign => {
                self.pos += 1; self.expect(&TokenKind::Key)?;
                self.expect(&TokenKind::LParen)?;
                let columns = self.parse_name_list()?;
                self.expect(&TokenKind::RParen)?;
                self.expect(&TokenKind::References)?;
                let clause = self.parse_fk_clause()?;
                Ok(TableConstraint::ForeignKey { name, columns, clause })
            }
            other => Err(self.err(format!("expected table constraint, found {:?}", other))),
        }
    }

    fn parse_indexed_columns(&mut self) -> Result<Vec<IndexedColumn>, ParseError> {
        let mut cols = Vec::new();
        loop {
            let name = self.expect_name()?;
            let collate = if self.eat(&TokenKind::Collate) { Some(self.expect_name()?) } else { None };
            let asc = if self.eat(&TokenKind::Desc) { false } else { self.eat(&TokenKind::Asc); true };
            cols.push(IndexedColumn { name, collate, asc });
            if !self.eat(&TokenKind::Comma) { break; }
        }
        Ok(cols)
    }

    fn parse_fk_clause(&mut self) -> Result<ForeignKeyClause, ParseError> {
        let table = self.expect_name()?;
        let columns = if self.eat(&TokenKind::LParen) {
            let cols = self.parse_name_list()?;
            self.expect(&TokenKind::RParen)?;
            cols
        } else { Vec::new() };
        let mut on_delete = None;
        let mut on_update = None;
        loop {
            if self.eat(&TokenKind::On) {
                if self.eat(&TokenKind::Delete) {
                    on_delete = Some(self.parse_referential_action()?);
                } else {
                    self.expect(&TokenKind::Update)?;
                    on_update = Some(self.parse_referential_action()?);
                }
            } else if self.eat(&TokenKind::Match) {
                self.expect_name()?; // match type name — ignored
            } else {
                break;
            }
        }
        let deferrable = self.eat(&TokenKind::Deferrable)
            || (self.eat(&TokenKind::Not) && { self.eat(&TokenKind::Deferrable); false });
        let initially_deferred = self.eat(&TokenKind::Initially) && self.eat(&TokenKind::Deferred);
        Ok(ForeignKeyClause { table, columns, on_delete, on_update, deferrable, initially_deferred })
    }

    fn parse_referential_action(&mut self) -> Result<ReferentialAction, ParseError> {
        match self.peek().clone() {
            TokenKind::Set => {
                self.pos += 1;
                if self.eat(&TokenKind::Null) { Ok(ReferentialAction::SetNull) }
                else { self.expect(&TokenKind::Default)?; Ok(ReferentialAction::SetDefault) }
            }
            TokenKind::Cascade  => { self.pos += 1; Ok(ReferentialAction::Cascade) }
            TokenKind::Restrict => { self.pos += 1; Ok(ReferentialAction::Restrict) }
            TokenKind::No       => { self.pos += 1; self.expect_name()?; Ok(ReferentialAction::NoAction) }
            other => Err(self.err(format!("expected referential action, found {:?}", other))),
        }
    }

    // ── CREATE INDEX ──────────────────────────────────────────────────────────

    fn parse_create_index(&mut self, unique: bool) -> Result<Stmt, ParseError> {
        let if_not_exists = self.eat_if_not_exists();
        let (schema, name) = self.parse_schema_name()?;
        self.expect(&TokenKind::On)?;
        let table = self.expect_name()?;
        self.expect(&TokenKind::LParen)?;
        let columns = self.parse_indexed_columns()?;
        self.expect(&TokenKind::RParen)?;
        let where_ = if self.eat(&TokenKind::Where) { Some(self.parse_expr(0)?) } else { None };
        Ok(Stmt::CreateIndex(CreateIndexStmt { unique, if_not_exists, schema, name, table, columns, where_ }))
    }

    // ── DROP ─────────────────────────────────────────────────────────────────

    fn parse_drop(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Drop)?;
        match self.peek().clone() {
            TokenKind::Table => {
                self.pos += 1;
                let if_exists = self.eat_if_exists();
                let (schema, name) = self.parse_schema_name()?;
                Ok(Stmt::DropTable(DropTableStmt { if_exists, schema, name }))
            }
            TokenKind::Index => {
                self.pos += 1;
                let if_exists = self.eat_if_exists();
                let (schema, name) = self.parse_schema_name()?;
                Ok(Stmt::DropIndex(DropIndexStmt { if_exists, schema, name }))
            }
            TokenKind::View | TokenKind::Trigger => {
                while !matches!(self.peek(), TokenKind::Semi | TokenKind::Eof) { self.pos += 1; }
                Ok(Stmt::Empty)
            }
            other => Err(self.err(format!("expected TABLE or INDEX after DROP, found {:?}", other))),
        }
    }

    // ── BEGIN ─────────────────────────────────────────────────────────────────

    fn parse_begin(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Begin)?;
        let mode = match self.peek() {
            TokenKind::Deferred  => { self.pos += 1; Some(TransactionMode::Deferred) }
            TokenKind::Immediate => { self.pos += 1; Some(TransactionMode::Immediate) }
            TokenKind::Exclusive => { self.pos += 1; Some(TransactionMode::Exclusive) }
            _ => None,
        };
        self.eat(&TokenKind::Transaction);
        Ok(Stmt::Begin(mode))
    }

    // ── PRAGMA ────────────────────────────────────────────────────────────────

    fn parse_pragma(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Pragma)?;
        let (schema, name) = self.parse_schema_name()?;
        let value = if self.eat(&TokenKind::Eq) {
            Some(self.parse_pragma_value()?)
        } else if self.eat(&TokenKind::LParen) {
            let v = self.parse_pragma_value()?;
            self.expect(&TokenKind::RParen)?;
            Some(v)
        } else {
            None
        };
        Ok(Stmt::Pragma(PragmaStmt { schema, name, value }))
    }

    fn parse_pragma_value(&mut self) -> Result<PragmaValue, ParseError> {
        match self.peek().clone() {
            TokenKind::Integer(n) => { self.pos += 1; Ok(PragmaValue::Integer(n)) }
            TokenKind::Real(f)    => { self.pos += 1; Ok(PragmaValue::Real(f)) }
            TokenKind::Minus => {
                self.pos += 1;
                match self.peek().clone() {
                    TokenKind::Integer(n) => { self.pos += 1; Ok(PragmaValue::Integer(-n)) }
                    TokenKind::Real(f)    => { self.pos += 1; Ok(PragmaValue::Real(-f)) }
                    _ => Err(self.err("expected number after '-' in PRAGMA value")),
                }
            }
            TokenKind::String(s)  => { self.pos += 1; Ok(PragmaValue::Text(s)) }
            _ => {
                let s = self.expect_name()?;
                Ok(PragmaValue::Text(s))
            }
        }
    }

    // ── ATTACH / DETACH ───────────────────────────────────────────────────────

    fn parse_attach(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Attach)?;
        self.eat(&TokenKind::Database);
        let file = self.parse_expr(0)?;
        self.expect(&TokenKind::As)?;
        let schema = self.expect_name()?;
        Ok(Stmt::Attach { file, schema })
    }

    fn parse_detach(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Detach)?;
        self.eat(&TokenKind::Database);
        Ok(Stmt::Detach(self.expect_name()?))
    }

    // ── Expressions (Pratt) ───────────────────────────────────────────────────

    fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;

        loop {
            // Postfix / special forms
            left = match self.peek() {
                TokenKind::Isnull => {
                    self.pos += 1;
                    Expr::IsNull { not: false, expr: Box::new(left) }
                }
                TokenKind::Notnull => {
                    self.pos += 1;
                    Expr::IsNull { not: true, expr: Box::new(left) }
                }
                TokenKind::Is if matches!(self.peek_at(1), TokenKind::Not) && matches!(self.peek_at(2), TokenKind::Null) => {
                    self.pos += 3;
                    Expr::IsNull { not: true, expr: Box::new(left) }
                }
                TokenKind::Is if matches!(self.peek_at(1), TokenKind::Null) => {
                    self.pos += 2;
                    Expr::IsNull { not: false, expr: Box::new(left) }
                }
                TokenKind::Collate => {
                    self.pos += 1;
                    let collation = self.expect_name()?;
                    Expr::Collate { expr: Box::new(left), collation }
                }
                TokenKind::Not if matches!(self.peek_at(1), TokenKind::Null) => {
                    self.pos += 2;
                    Expr::IsNull { not: true, expr: Box::new(left) }
                }
                _ => break,
            };
            // Postfix consumed, but we haven't checked min_prec — postfix always binds tighter
            continue;
        }

        // Re-do loop for infix operators
        loop {
            let (prec, right_prec) = match self.peek() {
                TokenKind::Or                                     => (PREC_OR,     PREC_OR + 1),
                TokenKind::And                                    => (PREC_AND,    PREC_AND + 1),
                TokenKind::Pipe                                   => (PREC_BITOR,  PREC_BITOR + 1),
                TokenKind::Amp                                    => (PREC_BITAND, PREC_BITAND + 1),
                TokenKind::LShift | TokenKind::RShift             => (PREC_SHIFT,  PREC_SHIFT + 1),
                TokenKind::Plus | TokenKind::Minus                => (PREC_ADD,    PREC_ADD + 1),
                TokenKind::Star | TokenKind::Slash | TokenKind::Percent => (PREC_MUL, PREC_MUL + 1),
                TokenKind::PipePipe                               => (PREC_CONCAT, PREC_CONCAT + 1),
                TokenKind::Lt | TokenKind::Le | TokenKind::Gt | TokenKind::Ge => (PREC_REL, PREC_REL + 1),
                TokenKind::Eq | TokenKind::Ne                     => (PREC_CMP,    PREC_CMP + 1),
                TokenKind::Is                                     => (PREC_CMP,    PREC_CMP + 1),
                TokenKind::In | TokenKind::Like | TokenKind::Glob
                | TokenKind::Match | TokenKind::Regexp            => (PREC_CMP,    PREC_CMP + 1),
                TokenKind::Between                                => (PREC_CMP,    PREC_CMP + 1),
                TokenKind::Not if matches!(self.peek_at(1),
                    TokenKind::In | TokenKind::Like | TokenKind::Glob
                    | TokenKind::Match | TokenKind::Regexp | TokenKind::Between) => (PREC_CMP, PREC_CMP + 1),
                // Postfix handled above; stop here
                TokenKind::Isnull | TokenKind::Notnull | TokenKind::Collate => break,
                _ => break,
            };
            if prec < min_prec { break; }

            left = self.parse_infix(left, right_prec)?;
        }
        Ok(left)
    }

    fn parse_infix(&mut self, left: Expr, right_prec: u8) -> Result<Expr, ParseError> {
        match self.peek().clone() {
            // Binary arithmetic / bitwise / concat
            TokenKind::Or      => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Or,     left: Box::new(left), right: Box::new(r) }) }
            TokenKind::And     => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::And,    left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Pipe    => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::BitOr,  left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Amp     => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::BitAnd, left: Box::new(left), right: Box::new(r) }) }
            TokenKind::LShift  => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::LShift, left: Box::new(left), right: Box::new(r) }) }
            TokenKind::RShift  => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::RShift, left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Plus    => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Add,    left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Minus   => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Sub,    left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Star    => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Mul,    left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Slash   => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Div,    left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Percent => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Rem,    left: Box::new(left), right: Box::new(r) }) }
            TokenKind::PipePipe=> { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Concat, left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Lt      => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Lt,     left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Le      => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Le,     left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Gt      => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Gt,     left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Ge      => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Ge,     left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Eq      => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Eq,     left: Box::new(left), right: Box::new(r) }) }
            TokenKind::Ne      => { self.pos += 1; let r = self.parse_expr(right_prec)?; Ok(Expr::BinOp { op: BinOp::Ne,     left: Box::new(left), right: Box::new(r) }) }
            // IS / IS NOT
            TokenKind::Is => {
                self.pos += 1;
                let not = self.eat(&TokenKind::Not);
                // IS NOT DISTINCT FROM / IS DISTINCT FROM
                if matches!(self.peek(), TokenKind::Distinct) {
                    self.pos += 1; self.expect(&TokenKind::From)?;
                    let r = self.parse_expr(right_prec)?;
                    Ok(Expr::Is { not: !not, left: Box::new(left), right: Box::new(r) })
                } else {
                    let r = self.parse_expr(right_prec)?;
                    Ok(Expr::Is { not, left: Box::new(left), right: Box::new(r) })
                }
            }
            // NOT IN / NOT LIKE / NOT GLOB / NOT BETWEEN / NOT MATCH / NOT REGEXP
            TokenKind::Not => {
                self.pos += 1;
                match self.peek().clone() {
                    TokenKind::In      => { self.pos += 1; self.parse_in(left, true) }
                    TokenKind::Like    => { self.pos += 1; self.parse_like(left, LikeOp::Like, true) }
                    TokenKind::Glob    => { self.pos += 1; self.parse_like(left, LikeOp::Glob, true) }
                    TokenKind::Match   => { self.pos += 1; self.parse_like(left, LikeOp::Match, true) }
                    TokenKind::Regexp  => { self.pos += 1; self.parse_like(left, LikeOp::Regexp, true) }
                    TokenKind::Between => { self.pos += 1; self.parse_between(left, true) }
                    other => Err(self.err(format!("expected IN/LIKE/GLOB/BETWEEN after NOT, found {:?}", other))),
                }
            }
            TokenKind::In      => { self.pos += 1; self.parse_in(left, false) }
            TokenKind::Like    => { self.pos += 1; self.parse_like(left, LikeOp::Like, false) }
            TokenKind::Glob    => { self.pos += 1; self.parse_like(left, LikeOp::Glob, false) }
            TokenKind::Match   => { self.pos += 1; self.parse_like(left, LikeOp::Match, false) }
            TokenKind::Regexp  => { self.pos += 1; self.parse_like(left, LikeOp::Regexp, false) }
            TokenKind::Between => { self.pos += 1; self.parse_between(left, false) }
            other => Err(self.err(format!("unexpected infix operator {:?}", other))),
        }
    }

    fn parse_in(&mut self, left: Expr, not: bool) -> Result<Expr, ParseError> {
        self.expect(&TokenKind::LParen)?;
        let set = if matches!(self.peek(), TokenKind::Select | TokenKind::With | TokenKind::Values) {
            InSet::Select(Box::new(self.parse_select_stmt()?))
        } else if matches!(self.peek(), TokenKind::RParen) {
            InSet::List(Vec::new())
        } else {
            InSet::List(self.parse_expr_list()?)
        };
        self.expect(&TokenKind::RParen)?;
        Ok(Expr::In { not, expr: Box::new(left), set })
    }

    fn parse_like(&mut self, left: Expr, op: LikeOp, not: bool) -> Result<Expr, ParseError> {
        let pattern = self.parse_expr(PREC_CMP + 1)?;
        let escape = if self.eat(&TokenKind::Escape) { Some(Box::new(self.parse_expr(PREC_CMP + 1)?)) } else { None };
        Ok(Expr::Like { not, op, expr: Box::new(left), pattern: Box::new(pattern), escape })
    }

    fn parse_between(&mut self, left: Expr, not: bool) -> Result<Expr, ParseError> {
        let lo = self.parse_expr(PREC_CMP + 1)?;
        self.expect(&TokenKind::And)?;
        let hi = self.parse_expr(PREC_CMP + 1)?;
        Ok(Expr::Between { not, expr: Box::new(left), lo: Box::new(lo), hi: Box::new(hi) })
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().clone() {
            TokenKind::Minus => { self.pos += 1; Ok(Expr::UnaryOp { op: UnaryOp::Neg, expr: Box::new(self.parse_unary()?) }) }
            TokenKind::Plus  => { self.pos += 1; Ok(Expr::UnaryOp { op: UnaryOp::Pos, expr: Box::new(self.parse_unary()?) }) }
            TokenKind::Tilde => { self.pos += 1; Ok(Expr::UnaryOp { op: UnaryOp::BitNot, expr: Box::new(self.parse_unary()?) }) }
            TokenKind::Not   => { self.pos += 1; Ok(Expr::UnaryOp { op: UnaryOp::Not, expr: Box::new(self.parse_unary()?) }) }
            _                => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut e = self.parse_primary()?;
        loop {
            e = match self.peek() {
                TokenKind::Isnull => { self.pos += 1; Expr::IsNull { not: false, expr: Box::new(e) } }
                TokenKind::Notnull => { self.pos += 1; Expr::IsNull { not: true, expr: Box::new(e) } }
                TokenKind::Collate => {
                    self.pos += 1;
                    let collation = self.expect_name()?;
                    Expr::Collate { expr: Box::new(e), collation }
                }
                _ => break,
            };
        }
        Ok(e)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().clone() {
            // Literals
            TokenKind::Integer(n) => { self.pos += 1; Ok(Expr::Literal(Literal::Integer(n))) }
            TokenKind::Real(f)    => { self.pos += 1; Ok(Expr::Literal(Literal::Real(f))) }
            TokenKind::String(s)  => { self.pos += 1; Ok(Expr::Literal(Literal::Text(s))) }
            TokenKind::Blob(b)    => { self.pos += 1; Ok(Expr::Literal(Literal::Blob(b))) }
            TokenKind::Null       => { self.pos += 1; Ok(Expr::Literal(Literal::Null)) }
            TokenKind::True       => { self.pos += 1; Ok(Expr::Literal(Literal::True)) }
            TokenKind::False      => { self.pos += 1; Ok(Expr::Literal(Literal::False)) }
            TokenKind::CurrentDate      => { self.pos += 1; Ok(Expr::Literal(Literal::CurrentDate)) }
            TokenKind::CurrentTime      => { self.pos += 1; Ok(Expr::Literal(Literal::CurrentTime)) }
            TokenKind::CurrentTimestamp => { self.pos += 1; Ok(Expr::Literal(Literal::CurrentTimestamp)) }

            TokenKind::BindParam(raw) => {
                let raw = raw.clone();
                self.pos += 1;
                let param = if raw == "?" {
                    BindParam::Positional
                } else if let Some(digits) = raw.strip_prefix('?') {
                    BindParam::PositionalN(digits.parse().unwrap_or(1))
                } else {
                    BindParam::Named(raw) // $i2, :name, @name — full string including prefix
                };
                Ok(Expr::BindParam(param))
            }
            // CAST
            TokenKind::Cast => {
                self.pos += 1;
                self.expect(&TokenKind::LParen)?;
                let expr = self.parse_expr(0)?;
                self.expect(&TokenKind::As)?;
                let type_name = self.try_parse_type_name()
                    .ok_or_else(|| self.err("expected type name in CAST"))?;
                self.expect(&TokenKind::RParen)?;
                Ok(Expr::Cast { expr: Box::new(expr), type_name })
            }

            // CASE
            TokenKind::Case => {
                self.pos += 1;
                let base = if !matches!(self.peek(), TokenKind::When) {
                    Some(Box::new(self.parse_expr(0)?))
                } else { None };
                let mut branches = Vec::new();
                while self.eat(&TokenKind::When) {
                    let cond = self.parse_expr(0)?;
                    self.expect(&TokenKind::Then)?;
                    let result = self.parse_expr(0)?;
                    branches.push((cond, result));
                }
                let else_ = if self.eat(&TokenKind::Else) { Some(Box::new(self.parse_expr(0)?)) } else { None };
                self.expect(&TokenKind::End)?;
                Ok(Expr::Case { base, branches, else_ })
            }

            // EXISTS
            TokenKind::Exists => {
                self.pos += 1;
                self.expect(&TokenKind::LParen)?;
                let sel = self.parse_select_stmt()?;
                self.expect(&TokenKind::RParen)?;
                Ok(Expr::Exists(Box::new(sel)))
            }

            // RAISE
            TokenKind::Raise => {
                self.pos += 1;
                self.expect(&TokenKind::LParen)?;
                let action = match self.peek().clone() {
                    TokenKind::Ignore   => { self.pos += 1; RaiseAction::Ignore }
                    TokenKind::Rollback => { self.pos += 1; RaiseAction::Rollback }
                    TokenKind::Abort    => { self.pos += 1; RaiseAction::Abort }
                    TokenKind::Fail     => { self.pos += 1; RaiseAction::Fail }
                    other => return Err(self.err(format!("expected raise action, found {:?}", other))),
                };
                let message = if self.eat(&TokenKind::Comma) {
                    Some(match self.advance() {
                        TokenKind::String(s) => s,
                        _ => return Err(self.err("expected string message in RAISE")),
                    })
                } else { None };
                self.expect(&TokenKind::RParen)?;
                Ok(Expr::Raise { action, message })
            }

            // Parenthesised expression or subquery
            TokenKind::LParen => {
                self.pos += 1;
                if matches!(self.peek(), TokenKind::Select | TokenKind::With | TokenKind::Values) {
                    let sel = self.parse_select_stmt()?;
                    self.expect(&TokenKind::RParen)?;
                    return Ok(Expr::Subquery(Box::new(sel)));
                }
                let e = self.parse_expr(0)?;
                self.expect(&TokenKind::RParen)?;
                Ok(e)
            }

            // Identifier, qualified name, or function call
            _ => {
                let name = self.expect_name()
                    .map_err(|_| self.err(format!("expected expression, found {:?}", self.peek())))?;

                // Function call: name(...)
                if self.eat(&TokenKind::LParen) {
                    return self.parse_function_call(name);
                }

                // Qualified: schema.table.column or table.column or column
                if self.eat(&TokenKind::Dot) {
                    let second = self.expect_name()?;
                    if self.eat(&TokenKind::Dot) {
                        let third = self.expect_name()?;
                        // schema.table.column
                        return Ok(Expr::Column { schema: Some(name), table: Some(second), name: third });
                    }
                    return Ok(Expr::Column { schema: None, table: Some(name), name: second });
                }

                Ok(Expr::Column { schema: None, table: None, name })
            }
        }
    }

    fn parse_function_call(&mut self, name: String) -> Result<Expr, ParseError> {
        // COUNT(*) or func(*)
        if self.eat(&TokenKind::Star) {
            self.expect(&TokenKind::RParen)?;
            let filter = if self.eat(&TokenKind::Filter) {
                self.expect(&TokenKind::LParen)?;
                self.expect(&TokenKind::Where)?;
                let e = self.parse_expr(0)?;
                self.expect(&TokenKind::RParen)?;
                Some(Box::new(e))
            } else { None };
            return Ok(Expr::FunctionCall { name, distinct: false, args: Vec::new(), filter });
        }
        let distinct = self.eat(&TokenKind::Distinct);
        let args = if matches!(self.peek(), TokenKind::RParen) {
            Vec::new()
        } else {
            self.parse_expr_list()?
        };
        self.expect(&TokenKind::RParen)?;
        let filter = if self.eat(&TokenKind::Filter) {
            self.expect(&TokenKind::LParen)?;
            self.expect(&TokenKind::Where)?;
            let e = self.parse_expr(0)?;
            self.expect(&TokenKind::RParen)?;
            Some(Box::new(e))
        } else { None };
        // Skip OVER clause (window function)
        if self.eat(&TokenKind::Over) {
            if self.eat(&TokenKind::LParen) { self.skip_balanced_paren()?; }
            else { self.expect_name()?; }
        }
        Ok(Expr::FunctionCall { name, distinct, args, filter })
    }

    /// Parse a literal-only expression (for DEFAULT values).
    fn parse_literal_expr(&mut self) -> Result<Expr, ParseError> {
        match self.peek().clone() {
            TokenKind::Plus  => { self.pos += 1; self.parse_literal_expr() }
            TokenKind::Minus => {
                self.pos += 1;
                match self.peek().clone() {
                    TokenKind::Integer(n) => { self.pos += 1; Ok(Expr::Literal(Literal::Integer(-n))) }
                    TokenKind::Real(f)    => { self.pos += 1; Ok(Expr::Literal(Literal::Real(-f))) }
                    _ => Err(self.err("expected number after '-'")),
                }
            }
            _ => self.parse_primary(),
        }
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn parse_expr_list(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut exprs = Vec::new();
        loop {
            exprs.push(self.parse_expr(0)?);
            if !self.eat(&TokenKind::Comma) { break; }
        }
        Ok(exprs)
    }

    fn parse_name_list(&mut self) -> Result<Vec<String>, ParseError> {
        let mut names = Vec::new();
        loop {
            names.push(self.expect_name()?);
            if !self.eat(&TokenKind::Comma) { break; }
        }
        Ok(names)
    }

    /// Parse `schema.name` or just `name`, returning `(Option<schema>, name)`.
    fn parse_schema_name(&mut self) -> Result<(Option<String>, String), ParseError> {
        let first = self.expect_name()?;
        if self.eat(&TokenKind::Dot) {
            Ok((Some(first), self.expect_name()?))
        } else {
            Ok((None, first))
        }
    }

    fn parse_conflict_action(&mut self) -> Result<ConflictAction, ParseError> {
        match self.peek().clone() {
            TokenKind::Rollback => { self.pos += 1; Ok(ConflictAction::Rollback) }
            TokenKind::Abort    => { self.pos += 1; Ok(ConflictAction::Abort) }
            TokenKind::Fail     => { self.pos += 1; Ok(ConflictAction::Fail) }
            TokenKind::Ignore   => { self.pos += 1; Ok(ConflictAction::Ignore) }
            TokenKind::Replace  => { self.pos += 1; Ok(ConflictAction::Replace) }
            other => Err(self.err(format!("expected conflict action, found {:?}", other))),
        }
    }

    fn try_parse_on_conflict(&mut self) -> Result<Option<ConflictAction>, ParseError> {
        if self.eat(&TokenKind::On) {
            self.expect(&TokenKind::Conflict)?;
            Ok(Some(self.parse_conflict_action()?))
        } else { Ok(None) }
    }

    fn parse_returning(&mut self) -> Result<Vec<ResultColumn>, ParseError> {
        if self.eat(&TokenKind::Returning) {
            self.parse_result_columns()
        } else { Ok(Vec::new()) }
    }

    fn eat_if_not_exists(&mut self) -> bool {
        if matches!(self.peek(), TokenKind::If)
            && matches!(self.peek_at(1), TokenKind::Not)
            && matches!(self.peek_at(2), TokenKind::Exists)
        {
            self.pos += 3; true
        } else { false }
    }

    fn eat_if_exists(&mut self) -> bool {
        if matches!(self.peek(), TokenKind::If) && matches!(self.peek_at(1), TokenKind::Exists) {
            self.pos += 2; true
        } else { false }
    }

    /// Skip a balanced parenthesised group, assuming the opening `(` was already consumed.
    fn skip_balanced_paren(&mut self) -> Result<(), ParseError> {
        let mut depth = 1usize;
        loop {
            match self.peek() {
                TokenKind::LParen => { self.pos += 1; depth += 1; }
                TokenKind::RParen => {
                    self.pos += 1;
                    depth -= 1;
                    if depth == 0 { return Ok(()); }
                }
                TokenKind::Eof => return Err(self.err("unterminated parenthesised group")),
                _ => { self.pos += 1; }
            }
        }
    }
}

// ── Keyword-as-identifier map ─────────────────────────────────────────────────
//
// SQLite allows a large set of keywords to be used as bare identifier names.
// Any keyword not listed here as "reserved" can be a name.

fn kw_as_name(k: &TokenKind) -> Option<String> {
    let s = match k {
        TokenKind::Abort    => "abort",     TokenKind::Action    => "action",
        TokenKind::Add      => "add",       TokenKind::After     => "after",
        TokenKind::Always   => "always",    TokenKind::Analyze   => "analyze",
        TokenKind::Asc      => "asc",       TokenKind::Attach    => "attach",
        TokenKind::Before   => "before",    TokenKind::Begin     => "begin",
        TokenKind::By       => "by",        TokenKind::Cascade   => "cascade",
        TokenKind::Cast     => "cast",      TokenKind::Column    => "column",
        TokenKind::Commit   => "commit",    TokenKind::Conflict  => "conflict",
        TokenKind::Current  => "current",   TokenKind::Database  => "database",
        TokenKind::Deferred => "deferred",  TokenKind::Desc      => "desc",
        TokenKind::Detach   => "detach",    TokenKind::Do        => "do",
        TokenKind::Each     => "each",      TokenKind::End       => "end",
        TokenKind::Escape   => "escape",    TokenKind::Exclude   => "exclude",
        TokenKind::Exclusive=> "exclusive", TokenKind::Explain   => "explain",
        TokenKind::Fail     => "fail",      TokenKind::Filter    => "filter",
        TokenKind::First    => "first",     TokenKind::Following => "following",
        TokenKind::For      => "for",       TokenKind::Full      => "full",
        TokenKind::Generated=> "generated", TokenKind::Groups    => "groups",
        TokenKind::If       => "if",        TokenKind::Ignore    => "ignore",
        TokenKind::Immediate=> "immediate", TokenKind::Index     => "index",
        TokenKind::Indexed  => "indexed",   TokenKind::Initially => "initially",
        TokenKind::Instead  => "instead",   TokenKind::Key       => "key",
        TokenKind::Last     => "last",      TokenKind::Limit     => "limit",
        TokenKind::Materialized => "materialized",
        TokenKind::No       => "no",        TokenKind::Nothing   => "nothing",
        TokenKind::Nulls    => "nulls",     TokenKind::Of        => "of",
        TokenKind::Offset   => "offset",    TokenKind::Others    => "others",
        TokenKind::Over     => "over",      TokenKind::Partition => "partition",
        TokenKind::Plan     => "plan",      TokenKind::Pragma    => "pragma",
        TokenKind::Preceding=> "preceding", TokenKind::Query     => "query",
        TokenKind::Raise    => "raise",     TokenKind::Range     => "range",
        TokenKind::Recursive=> "recursive", TokenKind::Reindex   => "reindex",
        TokenKind::Release  => "release",   TokenKind::Rename    => "rename",
        TokenKind::Replace  => "replace",   TokenKind::Restrict  => "restrict",
        TokenKind::Returning=> "returning", TokenKind::Rollback  => "rollback",
        TokenKind::Row      => "row",       TokenKind::Rows      => "rows",
        TokenKind::Savepoint=> "savepoint", TokenKind::Temp      => "temp",
        TokenKind::Temporary=> "temporary", TokenKind::Ties      => "ties",
        TokenKind::Transaction => "transaction",
        TokenKind::Trigger  => "trigger",   TokenKind::Unbounded => "unbounded",
        TokenKind::Vacuum   => "vacuum",    TokenKind::View      => "view",
        TokenKind::Virtual  => "virtual",   TokenKind::Window    => "window",
        TokenKind::Without  => "without",
        _ => return None,
    };
    Some(s.to_string())
}

/// Returns true if the token could be the start of an implicit alias name
/// (i.e., a bare identifier that isn't a clause keyword).
fn is_table_alias_start(k: &TokenKind) -> bool {
    matches!(k, TokenKind::Identifier(_))
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a SQL string into a list of statements.
pub fn parse_sql(sql: &str) -> Result<Vec<Stmt>, ParseError> {
    let tokens = tokenize(sql).map_err(|e| ParseError { message: e, offset: 0 })?;
    Parser::new(tokens).parse_stmts()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_one(sql: &str) -> Stmt {
        let mut stmts = parse_sql(sql).unwrap_or_else(|e| panic!("parse failed: {e}"));
        assert_eq!(stmts.len(), 1);
        stmts.remove(0)
    }

    #[test]
    fn select_star() {
        let s = parse_one("SELECT * FROM users");
        assert!(matches!(s, Stmt::Select(_)));
    }

    #[test]
    fn select_expr() {
        let s = parse_one("SELECT 1 + 2");
        assert!(matches!(s, Stmt::Select(_)));
    }

    #[test]
    fn select_where() {
        let s = parse_one("SELECT id, name FROM users WHERE id = 1");
        assert!(matches!(s, Stmt::Select(_)));
    }

    #[test]
    fn select_join() {
        let s = parse_one("SELECT a.id, b.name FROM a LEFT JOIN b ON a.id = b.aid");
        assert!(matches!(s, Stmt::Select(_)));
    }

    #[test]
    fn select_subquery() {
        let s = parse_one("SELECT * FROM (SELECT id FROM t) sub");
        assert!(matches!(s, Stmt::Select(_)));
    }

    #[test]
    fn select_compound() {
        let s = parse_one("SELECT 1 UNION ALL SELECT 2");
        assert!(matches!(s, Stmt::Select(_)));
    }

    #[test]
    fn select_with() {
        let s = parse_one("WITH cte AS (SELECT 1) SELECT * FROM cte");
        assert!(matches!(s, Stmt::Select(_)));
    }

    #[test]
    fn select_order_limit() {
        let s = parse_one("SELECT * FROM t ORDER BY id DESC LIMIT 10 OFFSET 5");
        assert!(matches!(s, Stmt::Select(_)));
    }

    #[test]
    fn insert_values() {
        let s = parse_one("INSERT INTO t (a, b) VALUES (1, 'hello')");
        assert!(matches!(s, Stmt::Insert(_)));
    }

    #[test]
    fn insert_select() {
        let s = parse_one("INSERT INTO t SELECT * FROM other");
        assert!(matches!(s, Stmt::Insert(_)));
    }

    #[test]
    fn update_stmt() {
        let s = parse_one("UPDATE t SET a = 1, b = 'x' WHERE id = 42");
        assert!(matches!(s, Stmt::Update(_)));
    }

    #[test]
    fn delete_stmt() {
        let s = parse_one("DELETE FROM t WHERE id = 1");
        assert!(matches!(s, Stmt::Delete(_)));
    }

    #[test]
    fn create_table() {
        let s = parse_one("CREATE TABLE t (id INTEGER PRIMARY KEY, name TEXT NOT NULL)");
        assert!(matches!(s, Stmt::CreateTable(_)));
    }

    #[test]
    fn create_table_fk() {
        let s = parse_one("CREATE TABLE t (id INTEGER, parent_id INTEGER REFERENCES p(id))");
        assert!(matches!(s, Stmt::CreateTable(_)));
    }

    #[test]
    fn create_index() {
        let s = parse_one("CREATE UNIQUE INDEX idx ON t (col1, col2)");
        assert!(matches!(s, Stmt::CreateIndex(_)));
    }

    #[test]
    fn drop_table() {
        let s = parse_one("DROP TABLE IF EXISTS t");
        assert!(matches!(s, Stmt::DropTable(_)));
    }

    #[test]
    fn begin_commit() {
        assert!(matches!(parse_one("BEGIN"), Stmt::Begin(_)));
        assert!(matches!(parse_one("BEGIN IMMEDIATE"), Stmt::Begin(_)));
        assert!(matches!(parse_one("COMMIT"), Stmt::Commit));
        assert!(matches!(parse_one("ROLLBACK"), Stmt::Rollback));
    }

    #[test]
    fn pragma_stmt() {
        assert!(matches!(parse_one("PRAGMA journal_mode"), Stmt::Pragma(_)));
        assert!(matches!(parse_one("PRAGMA journal_mode = wal"), Stmt::Pragma(_)));
    }

    #[test]
    fn pragma_main() {
        assert!(matches!(parse_one("PRAGMA 'main'.encoding"), Stmt::Pragma(_)));
    }

    #[test]
    fn expr_precedence() {
        // 1 + 2 * 3 should parse as 1 + (2 * 3)
        let s = parse_one("SELECT 1 + 2 * 3");
        if let Stmt::Select(sel) = s {
            if let ResultColumn::Expr { expr, .. } = &sel.core.columns[0] {
                if let Expr::BinOp { op: BinOp::Add, right, .. } = expr {
                    assert!(matches!(**right, Expr::BinOp { op: BinOp::Mul, .. }));
                } else { panic!("expected Add"); }
            }
        }
    }

    #[test]
    fn expr_in_list() {
        let s = parse_one("SELECT * FROM t WHERE id IN (1, 2, 3)");
        assert!(matches!(s, Stmt::Select(_)));
    }

    #[test]
    fn expr_between() {
        let s = parse_one("SELECT * FROM t WHERE x BETWEEN 1 AND 10");
        assert!(matches!(s, Stmt::Select(_)));
    }

    #[test]
    fn expr_case() {
        let s = parse_one("SELECT CASE x WHEN 1 THEN 'one' ELSE 'other' END FROM t");
        assert!(matches!(s, Stmt::Select(_)));
    }

    #[test]
    fn expr_function() {
        let s = parse_one("SELECT count(*), max(id) FROM t");
        assert!(matches!(s, Stmt::Select(_)));
    }

    #[test]
    fn multiple_stmts() {
        let stmts = parse_sql("SELECT 1; SELECT 2; SELECT 3").unwrap();
        assert_eq!(stmts.len(), 3);
    }
}
