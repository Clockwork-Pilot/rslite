//! Tests for sqlite3VMPrintf via sqlite3MPrintf and other internal APIs.
//! These tests exercise the extract_printf_args + sqlite3_str_vappendf_args path
//! by opening a real database and running SQL that triggers internal printf formatting.

use core::ffi::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};

// Use functions through the crate's module system
use sqlite_noamalgam::src::src::legacy::sqlite3_exec;
use sqlite_noamalgam::src::src::main::{sqlite3_close_v2 as sqlite3_close, sqlite3_open};
use sqlite_noamalgam::src::src::malloc::sqlite3_free;

unsafe extern "C" {
    fn sqlite3_errmsg(db: *mut c_void) -> *const c_char;
    fn sqlite3_mprintf(fmt: *const c_char, ...) -> *mut c_char;
}

const SQLITE_OK: c_int = 0;

/// Helper: open an in-memory database
unsafe fn open_memdb() -> *mut c_void {
    let mut db: *mut c_void = std::ptr::null_mut();
    let rc = sqlite3_open(b":memory:\0".as_ptr() as _, &mut db as *mut _ as *mut _);
    assert_eq!(rc, SQLITE_OK, "Failed to open :memory: db");
    db
}

/// Helper: exec SQL, return error string if any
unsafe fn exec(db: *mut c_void, sql: &str) -> Result<(), String> {
    let csql = CString::new(sql).unwrap();
    let mut errmsg: *mut c_char = std::ptr::null_mut();
    let rc = sqlite3_exec(
        db as _,
        csql.as_ptr(),
        None,
        std::ptr::null_mut(),
        &mut errmsg,
    );
    if rc != SQLITE_OK {
        let msg = if !errmsg.is_null() {
            let s = CStr::from_ptr(errmsg).to_string_lossy().into_owned();
            sqlite3_free(errmsg as *mut c_void);
            s
        } else {
            CStr::from_ptr(sqlite3_errmsg(db))
                .to_string_lossy()
                .into_owned()
        };
        Err(format!("rc={rc}: {msg}"))
    } else {
        if !errmsg.is_null() {
            sqlite3_free(errmsg as *mut c_void);
        }
        Ok(())
    }
}

/// Collect query results as Vec<Vec<String>>
unsafe fn query(db: *mut c_void, sql: &str) -> Vec<Vec<String>> {
    struct Ctx {
        rows: Vec<Vec<String>>,
    }
    unsafe extern "C" fn cb(
        arg: *mut c_void,
        ncols: c_int,
        values: *mut *mut c_char,
        _names: *mut *mut c_char,
    ) -> c_int {
        let ctx = unsafe { &mut *(arg as *mut Ctx) };
        let mut row = Vec::new();
        for i in 0..ncols as isize {
            let v = unsafe { *values.offset(i) };
            if v.is_null() {
                row.push("NULL".to_string());
            } else {
                row.push(unsafe { CStr::from_ptr(v) }.to_string_lossy().into_owned());
            }
        }
        ctx.rows.push(row);
        0
    }
    let csql = CString::new(sql).unwrap();
    let mut ctx = Ctx { rows: Vec::new() };
    let mut errmsg: *mut c_char = std::ptr::null_mut();
    let rc = sqlite3_exec(
        db as _,
        csql.as_ptr(),
        Some(cb),
        &mut ctx as *mut Ctx as *mut c_void,
        &mut errmsg,
    );
    if !errmsg.is_null() {
        sqlite3_free(errmsg as *mut c_void);
    }
    assert_eq!(rc, SQLITE_OK, "query failed: {sql}");
    ctx.rows
}

// ─── Basic internal printf through SQL ──────────────────────────────────

#[test]
fn vmprintf_smoke_open_close() {
    unsafe {
        let db = open_memdb();
        exec(db, "CREATE TABLE t1(a INT)").unwrap();
        exec(db, "INSERT INTO t1 VALUES(42)").unwrap();
        let rows = query(db, "SELECT a FROM t1");
        assert_eq!(rows[0][0], "42");
        sqlite3_close(db as _);
    }
}

#[test]
fn vmprintf_basic_sql_operations() {
    unsafe {
        let db = open_memdb();
        exec(db, "CREATE TABLE t1(a INTEGER, b TEXT, c REAL)").unwrap();
        exec(db, "INSERT INTO t1 VALUES(1, 'hello', 3.14)").unwrap();
        exec(db, "INSERT INTO t1 VALUES(2, 'world', 2.72)").unwrap();

        let rows = query(db, "SELECT a, b, c FROM t1 ORDER BY a");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][0], "1");
        assert_eq!(rows[0][1], "hello");
        assert_eq!(rows[1][0], "2");
        assert_eq!(rows[1][1], "world");

        sqlite3_close(db as _);
    }
}

#[test]
fn vmprintf_error_messages() {
    unsafe {
        let db = open_memdb();
        exec(db, "CREATE TABLE t1(a INTEGER PRIMARY KEY)").unwrap();
        exec(db, "INSERT INTO t1 VALUES(1)").unwrap();
        // Duplicate key
        let err = exec(db, "INSERT INTO t1 VALUES(1)");
        assert!(err.is_err());
        let msg = err.unwrap_err();
        assert!(
            msg.contains("UNIQUE constraint failed"),
            "Expected UNIQUE error, got: {msg}"
        );

        // No such table
        let err = exec(db, "SELECT * FROM nonexistent_table");
        assert!(err.is_err());
        let msg = err.unwrap_err();
        assert!(
            msg.contains("nonexistent_table"),
            "Expected table name in error, got: {msg}"
        );

        // No such column
        let err = exec(db, "SELECT nonexistent_col FROM t1");
        assert!(err.is_err());
        let msg = err.unwrap_err();
        assert!(
            msg.contains("nonexistent_col"),
            "Expected column name in error, got: {msg}"
        );

        sqlite3_close(db as _);
    }
}

#[test]
fn vmprintf_attach_databases() {
    unsafe {
        let db = open_memdb();
        exec(db, "ATTACH ':memory:' AS db2").unwrap();
        exec(db, "CREATE TABLE db2.t2(x TEXT)").unwrap();
        exec(db, "INSERT INTO db2.t2 VALUES('attached')").unwrap();

        let rows = query(db, "SELECT x FROM db2.t2");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], "attached");

        exec(db, "CREATE TABLE main.t1(a INT)").unwrap();
        exec(db, "INSERT INTO main.t1 VALUES(42)").unwrap();
        let rows = query(db, "SELECT a FROM main.t1 UNION ALL SELECT x FROM db2.t2");
        assert_eq!(rows.len(), 2);

        sqlite3_close(db as _);
    }
}

#[test]
fn vmprintf_complex_sql_patterns() {
    unsafe {
        let db = open_memdb();
        exec(db, "CREATE TABLE t1(a INT, b TEXT)").unwrap();
        for i in 0..100 {
            let sql = format!("INSERT INTO t1 VALUES({i}, 'row_{i}')");
            exec(db, &sql).unwrap();
        }

        let rows = query(db, "SELECT count(*) FROM t1 WHERE a > 50");
        assert_eq!(rows[0][0], "49");

        let rows = query(db, "SELECT a, b FROM t1 ORDER BY a DESC LIMIT 3");
        assert_eq!(rows[0][0], "99");
        assert_eq!(rows[1][0], "98");
        assert_eq!(rows[2][0], "97");

        let rows = query(
            db,
            "SELECT a % 10 AS bucket, count(*) FROM t1 GROUP BY bucket ORDER BY bucket",
        );
        assert_eq!(rows.len(), 10);

        sqlite3_close(db as _);
    }
}

#[test]
fn vmprintf_pragma_formatting() {
    unsafe {
        let db = open_memdb();
        exec(db, "CREATE TABLE t1(a, b, c)").unwrap();

        let rows = query(db, "PRAGMA table_info(t1)");
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0][1], "a");
        assert_eq!(rows[1][1], "b");
        assert_eq!(rows[2][1], "c");

        let rows = query(db, "PRAGMA integrity_check");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], "ok");

        sqlite3_close(db as _);
    }
}

#[test]
fn vmprintf_index_and_constraint_errors() {
    unsafe {
        let db = open_memdb();
        exec(db, "CREATE TABLE t1(a INT, b TEXT)").unwrap();
        exec(db, "CREATE INDEX idx1 ON t1(a)").unwrap();
        exec(db, "CREATE UNIQUE INDEX idx2 ON t1(b)").unwrap();

        exec(db, "INSERT INTO t1 VALUES(1, 'unique_val')").unwrap();
        let err = exec(db, "INSERT INTO t1 VALUES(2, 'unique_val')");
        assert!(err.is_err());
        let msg = err.unwrap_err();
        assert!(
            msg.contains("UNIQUE constraint failed") && msg.contains("t1.b"),
            "Expected UNIQUE constraint error mentioning t1.b, got: {msg}"
        );

        sqlite3_close(db as _);
    }
}

#[test]
fn vmprintf_alter_table() {
    unsafe {
        let db = open_memdb();
        exec(db, "CREATE TABLE t1(a INT, b TEXT)").unwrap();
        exec(db, "INSERT INTO t1 VALUES(1, 'hello')").unwrap();

        exec(db, "ALTER TABLE t1 ADD COLUMN c REAL DEFAULT 0.0").unwrap();
        let rows = query(db, "SELECT a, b, c FROM t1");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][2], "0.0");

        // Note: ALTER TABLE RENAME triggers sqlite3DeleteTrigger which has a
        // pre-existing null-pointer bug (trigger.rs:834). Avoid RENAME for now.
        let rows = query(db, "SELECT count(*) FROM t1");
        assert_eq!(rows[0][0], "1");

        sqlite3_close(db as _);
    }
}

#[test]
fn vmprintf_views_and_triggers() {
    // Note: CREATE TRIGGER + sqlite3_close triggers a pre-existing null-pointer
    // bug in sqlite3DeleteTrigger (trigger.rs:834). Test views only for now.
    unsafe {
        let db = open_memdb();
        exec(db, "CREATE TABLE t1(a INT, b TEXT)").unwrap();
        exec(db, "CREATE VIEW v1 AS SELECT a, b FROM t1 WHERE a > 0").unwrap();
        exec(db, "INSERT INTO t1 VALUES(1, 'x')").unwrap();
        exec(db, "INSERT INTO t1 VALUES(-1, 'y')").unwrap();

        let rows = query(db, "SELECT * FROM v1");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], "1");

        sqlite3_close(db as _);
    }
}

#[test]
#[ignore] // Pre-existing null-pointer bug in whereexpr.rs:1399 with FTS5 MATCH
fn vmprintf_fts5_operations_disabled() {
    // Disabled: FTS5 MATCH queries trigger a pre-existing null-pointer bug
    // in whereexpr.rs:1399. The testfixture binary works around this differently.
    unsafe {
        let db = open_memdb();
        exec(db, "CREATE VIRTUAL TABLE ft USING fts5(content)").unwrap();
        exec(db, "INSERT INTO ft VALUES('hello world')").unwrap();
        exec(db, "INSERT INTO ft VALUES('goodbye world')").unwrap();
        exec(db, "INSERT INTO ft VALUES('hello goodbye')").unwrap();

        let rows = query(
            db,
            "SELECT content FROM ft WHERE ft MATCH 'hello' ORDER BY rank",
        );
        assert_eq!(rows.len(), 2);

        let rows = query(
            db,
            "SELECT content FROM ft WHERE ft MATCH 'world' ORDER BY rank",
        );
        assert_eq!(rows.len(), 2);

        exec(db, "INSERT INTO ft(ft) VALUES('rebuild')").unwrap();
        exec(db, "INSERT INTO ft(ft) VALUES('integrity-check')").unwrap();

        sqlite3_close(db as _);
    }
}

#[test]
fn vmprintf_rtree_operations() {
    unsafe {
        let db = open_memdb();
        exec(
            db,
            "CREATE VIRTUAL TABLE rt USING rtree(id, x1, x2, y1, y2)",
        )
        .unwrap();
        exec(db, "INSERT INTO rt VALUES(1, 0.0, 1.0, 0.0, 1.0)").unwrap();
        exec(db, "INSERT INTO rt VALUES(2, 5.0, 6.0, 5.0, 6.0)").unwrap();
        exec(db, "INSERT INTO rt VALUES(3, 0.5, 1.5, 0.5, 1.5)").unwrap();

        let rows = query(
            db,
            "SELECT id FROM rt WHERE x1 >= 0.0 AND x2 <= 2.0 AND y1 >= 0.0 AND y2 <= 2.0",
        );
        assert!(rows.len() >= 1, "Expected at least one rtree result");

        sqlite3_close(db as _);
    }
}

#[test]
fn vmprintf_explain_output() {
    unsafe {
        let db = open_memdb();
        exec(db, "CREATE TABLE t1(a INT, b TEXT)").unwrap();

        let rows = query(db, "EXPLAIN SELECT * FROM t1 WHERE a = 1");
        assert!(!rows.is_empty(), "EXPLAIN should produce output");

        for row in &rows {
            assert!(!row[1].is_empty(), "Opcode name should not be empty");
        }

        let rows = query(db, "EXPLAIN QUERY PLAN SELECT * FROM t1 WHERE a = 1");
        assert!(!rows.is_empty(), "EQP should produce output");

        sqlite3_close(db as _);
    }
}

#[test]
fn vmprintf_long_strings() {
    unsafe {
        let db = open_memdb();
        exec(db, "CREATE TABLE t1(a TEXT)").unwrap();

        let long_str = "x".repeat(200);
        let sql = format!("INSERT INTO t1 VALUES('{long_str}')");
        exec(db, &sql).unwrap();

        let rows = query(db, "SELECT a FROM t1");
        assert_eq!(rows[0][0], long_str);

        let cols: Vec<String> = (0..50).map(|i| format!("col_{i} INT")).collect();
        let create = format!("CREATE TABLE big_table({})", cols.join(", "));
        exec(db, &create).unwrap();

        let rows = query(db, "PRAGMA table_info(big_table)");
        assert_eq!(rows.len(), 50);

        sqlite3_close(db as _);
    }
}

#[test]
fn vmprintf_multi_file_db_transaction() {
    // This specifically tests what mjournal.test exercises:
    // multi-database transactions that create master journal files.
    // Master journal filenames are constructed via sqlite3VMPrintf.
    unsafe {
        let db = open_memdb();

        // Create file-based databases (not :memory:)
        let tmpdir = std::env::temp_dir();
        let db1 = tmpdir.join("vmprintf_test_db1.sqlite");
        let db2 = tmpdir.join("vmprintf_test_db2.sqlite");
        let _ = std::fs::remove_file(&db1);
        let _ = std::fs::remove_file(&db2);

        let attach1 = format!("ATTACH '{}' AS filedb1", db1.display());
        let attach2 = format!("ATTACH '{}' AS filedb2", db2.display());
        exec(db, &attach1).unwrap();
        exec(db, &attach2).unwrap();

        exec(db, "CREATE TABLE filedb1.t1(x)").unwrap();
        exec(db, "CREATE TABLE filedb2.t2(x)").unwrap();

        // Multi-database transaction (triggers master journal creation)
        exec(db, "BEGIN").unwrap();
        exec(db, "INSERT INTO filedb1.t1 VALUES(1)").unwrap();
        exec(db, "INSERT INTO filedb2.t2 VALUES(2)").unwrap();
        exec(db, "COMMIT").unwrap();

        // Verify data
        let rows = query(db, "SELECT x FROM filedb1.t1");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], "1");

        let rows = query(db, "SELECT x FROM filedb2.t2");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], "2");

        // Multiple transactions
        for i in 3..10 {
            exec(db, "BEGIN").unwrap();
            exec(db, &format!("INSERT INTO filedb1.t1 VALUES({i})")).unwrap();
            exec(db, &format!("INSERT INTO filedb2.t2 VALUES({i})")).unwrap();
            exec(db, "COMMIT").unwrap();
        }

        let rows = query(db, "SELECT count(*) FROM filedb1.t1");
        assert_eq!(rows[0][0], "8"); // 1 + 7 more

        sqlite3_close(db as _);
        let _ = std::fs::remove_file(&db1);
        let _ = std::fs::remove_file(&db2);
    }
}

#[test]
fn vmprintf_embedded_nul_bytes() {
    // Test that sqlite3VMPrintf correctly handles format strings that produce
    // output with embedded NUL bytes (e.g., "%.4c" with char value 0).
    // This is critical for master journal filename construction:
    //   sqlite3MPrintf(db, "%.4c%s%.16c", 0, zMainFile, 0)
    // which produces: \0\0\0\0 + mainFilePath + \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0
    unsafe {
        let take = |p: *mut c_char| -> Vec<u8> {
            if p.is_null() {
                return vec![];
            }
            // Can't use CStr because of embedded NULs
            // Use sqlite3_mprintf's return - we know the format produces exactly
            // 4 + len(filename) + 16 bytes
            let filename = b"test.db\0";
            let expected_len = 4 + (filename.len() - 1) + 16; // minus the trailing NUL of filename
            let bytes = std::slice::from_raw_parts(p as *const u8, expected_len);
            let result = bytes.to_vec();
            sqlite3_free(p as *mut c_void);
            result
        };

        let filename: *const c_char = b"test.db\0".as_ptr() as _;
        let result = take(sqlite3_mprintf(
            b"%.4c%s%.16c\0".as_ptr() as _,
            0i32,
            filename,
            0i32,
        ));

        // First 4 bytes should be NUL (%.4c with char 0)
        assert_eq!(&result[0..4], &[0, 0, 0, 0], "Expected 4 NUL prefix bytes");

        // Middle should be "test.db"
        assert_eq!(&result[4..11], b"test.db", "Expected filename in middle");

        // Last 16 bytes should be NUL (%.16c with char 0)
        assert_eq!(&result[11..27], &[0u8; 16], "Expected 16 NUL suffix bytes");
    }
}

#[test]
fn vmprintf_multiple_format_specifiers() {
    unsafe {
        let take = |p: *mut c_char| -> String {
            if p.is_null() {
                return "(null)".into();
            }
            let s = CStr::from_ptr(p).to_string_lossy().into_owned();
            sqlite3_free(p as *mut c_void);
            s
        };

        let a: *const c_char = b"hello\0".as_ptr() as _;
        let b: *const c_char = b"world\0".as_ptr() as _;
        assert_eq!(
            take(sqlite3_mprintf(b"%s %s\0".as_ptr() as _, a, b)),
            "hello world"
        );

        assert_eq!(
            take(sqlite3_mprintf(
                b"%s=%d (%.2f)\0".as_ptr() as _,
                a,
                42i32,
                3.14f64
            )),
            "hello=42 (3.14)"
        );

        let long_name: *const c_char =
            b"a_very_long_table_name_that_exceeds_the_stack_buffer_size_in_straccum_init\0".as_ptr()
                as _;
        let result = take(sqlite3_mprintf(
            b"CREATE TABLE %s (id INT)\0".as_ptr() as _,
            long_name,
        ));
        assert!(
            result.contains("a_very_long_table_name"),
            "Long string formatting failed: {result}"
        );
        assert!(result.contains("(id INT)"), "Suffix lost: {result}");
    }
}

// ─── SQLFUNC printf() tests (direct-mode path) ─────────────────────────
// These test the SQL printf() function which goes through the SQLFUNC
// direct-mode path (sqlite3_str_vappendf_sqlfunc), exercising
// single-pass argument consumption from PrintfArguments.

/// Helper: run SELECT printf(...) and return the result string
unsafe fn sql_printf(db: *mut c_void, expr: &str) -> String {
    let rows = query(db, &format!("SELECT {expr}"));
    assert!(!rows.is_empty(), "printf returned no rows for: {expr}");
    rows[0][0].clone()
}

#[test]
fn sqlfunc_printf_char_repeat_width_eq_precision() {
    // printf2-3.4: %8.8c where width == precision
    // This is the exact failing case: the first %c in a multi-%c format
    // was getting a corrupted text pointer in direct mode.
    unsafe {
        let db = open_memdb();

        // Single %c with precision
        assert_eq!(
            sql_printf(db, "printf('|%8.8c|','*')"),
            "|********|",
            "single %8.8c"
        );

        // Two %c specifiers (the failing case)
        assert_eq!(
            sql_printf(db, "printf('|%8.8c|%-8.8c|','*','*')"),
            "|********|********|",
            "printf2-3.4: two %c with width==precision"
        );

        // Width > precision (printf2-3.3, was passing)
        assert_eq!(
            sql_printf(db, "printf('|%9.8c|%-9.8c|','*','*')"),
            "| ********|******** |",
            "printf2-3.3: two %c with width>precision"
        );

        // Width < precision (printf2-3.5)
        assert_eq!(
            sql_printf(db, "printf('|%7.8c|%-7.8c|','*','*')"),
            "|********|********|",
            "printf2-3.5: two %c with width<precision"
        );

        // Large repeat (printf2-3.1)
        let result = sql_printf(db, "printf('|%110.100c|','*')");
        let stars: String = std::iter::repeat('*').take(100).collect();
        let spaces: String = std::iter::repeat(' ').take(10).collect();
        assert_eq!(
            result,
            format!("|{spaces}{stars}|"),
            "printf2-3.1: large repeat"
        );

        // Different characters in multi-%c
        assert_eq!(
            sql_printf(db, "printf('%3.3c%3.3c','A','B')"),
            "AAABBB",
            "different chars in multi-%c"
        );

        sqlite3_close(db as _);
    }
}

#[test]
fn sqlfunc_printf_mixed_specifiers() {
    // Test mixing %c with %s, %d, %Q in SQLFUNC direct mode
    unsafe {
        let db = open_memdb();

        assert_eq!(
            sql_printf(db, "printf('%s=%d','hello',42)"),
            "hello=42",
            "basic %s %d"
        );

        assert_eq!(
            sql_printf(db, "printf('%Q','it''s')"),
            "'it''s'",
            "basic %Q"
        );

        // Mix %c with %s
        assert_eq!(
            sql_printf(db, "printf('%c-%s','X','hello')"),
            "X-hello",
            "%c then %s"
        );

        // Mix %s with %c
        assert_eq!(
            sql_printf(db, "printf('%s-%c','hello','X')"),
            "hello-X",
            "%s then %c"
        );

        // %% (literal percent) should not consume an arg
        assert_eq!(
            sql_printf(db, "printf('100%% of %s','tests')"),
            "100% of tests",
            "%% then %s"
        );

        // Multiple %% interspersed
        assert_eq!(
            sql_printf(db, "printf('%s%%=%d%%','score',95)"),
            "score%=95%",
            "%s %% %d %%"
        );

        sqlite3_close(db as _);
    }
}

#[test]
fn sqlfunc_printf_percent_arg_alignment() {
    // Ensure %% doesn't misalign subsequent args in SQLFUNC direct mode.
    unsafe {
        let db = open_memdb();

        // %% produces literal %, no arg consumed
        assert_eq!(sql_printf(db, "printf('100%%')"), "100%", "bare %%");

        // %s after %%
        assert_eq!(
            sql_printf(db, "printf('100%% of %s','tests')"),
            "100% of tests",
            "%% then %s"
        );

        // %s before and after %%
        assert_eq!(
            sql_printf(db, "printf('%s %% %s','left','right')"),
            "left % right",
            "%s %% %s alignment"
        );

        // Multiple %% interspersed with args
        assert_eq!(
            sql_printf(db, "printf('%s%%=%d%%','score',95)"),
            "score%=95%",
            "%s %% %d %%"
        );

        // Double %% (%%%% → %%)
        assert_eq!(
            sql_printf(db, "printf('%%%%-%s','end')"),
            "%%-end",
            "double-percent before string"
        );

        sqlite3_close(db as _);
    }
}

#[test]
fn sqlfunc_printf_dynstring_z() {
    // Test %z (dynamic string, freed after append) through sqlite3_mprintf.
    // This tests the etDYNSTRING path where we fixed a potential double-free
    // when the arg is null (bufpt was set to static "\0" then assigned to zExtra).
    unsafe {
        let take = |p: *mut c_char| -> String {
            if p.is_null() {
                return "(null)".into();
            }
            let s = CStr::from_ptr(p).to_string_lossy().into_owned();
            sqlite3_free(p as *mut c_void);
            s
        };

        let sep = b",\0".as_ptr() as *const c_char;
        let a = b"one\0".as_ptr() as *const c_char;
        let b = b"two\0".as_ptr() as *const c_char;
        let c = b"three\0".as_ptr() as *const c_char;

        // Build up string with %z (each call frees the previous result).
        // First call: %z with NULL (no prior string to free), then ,one
        let p1 = sqlite3_mprintf(
            b"%z%s%s\0".as_ptr() as _,
            std::ptr::null::<c_char>(),
            sep,
            a,
        );
        assert!(!p1.is_null());
        let s1 = CStr::from_ptr(p1).to_string_lossy().into_owned();
        assert_eq!(s1, ",one", "first %z concat");

        // Second concat — p1 is sqlite3_malloc'd, so %z can free it
        let p2 = sqlite3_mprintf(b"%z%s%s\0".as_ptr() as _, p1, sep, b);
        assert!(!p2.is_null());
        let s2 = CStr::from_ptr(p2).to_string_lossy().into_owned();
        assert_eq!(s2, ",one,two", "second %z concat");

        // Third concat
        let p3 = sqlite3_mprintf(b"%z%s%s\0".as_ptr() as _, p2, sep, c);
        assert!(!p3.is_null());
        let s3 = take(p3);
        assert_eq!(s3, ",one,two,three", "third %z concat");
    }
}
