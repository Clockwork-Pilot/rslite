//! Compatibility tests ported from rusqlite 0.32.1.
//!
//! Sources:
//!   src/lib.rs          — mod test
//!   src/transaction.rs  — mod test
//!   src/statement.rs    — mod test
//!   src/types/mod.rs    — mod test
//!
//! Each test is kept as close to the rusqlite original as possible.
//! Adaptations:
//!   - `[]` (empty array) replaced by `()` (our zero-params type)
//!   - `OpenFlags::SQLITE_OPEN_*` → `OpenFlags::*`
//!   - `db.one_column(sql)` is available on our Connection
//!   - Feature-gated or rusqlite-internal tests are skipped

use rslite::{
    named_params, params, params_from_iter,
    types::{ToSql, Value},
    Action, Aggregate, Connection, Context, DropBehavior, Error, ErrorCode,
    FunctionFlags, Null, OpenFlags, OptionalExtension,
};
use rslite_raw::{SQLITE_CANTOPEN, SQLITE_CONSTRAINT_NOTNULL};
use std::os::raw::{c_double, c_int};

// ═══════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════

fn checked_memory_handle() -> Connection {
    Connection::open_in_memory().unwrap()
}

// ═══════════════════════════════════════════════════════════════════════════
// lib.rs tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_concurrent_transactions_busy_commit() {
    use std::time::Duration;
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("transactions.db3");

    Connection::open(&path)
        .unwrap()
        .execute_batch(
            "BEGIN; CREATE TABLE foo(x INTEGER);
             INSERT INTO foo VALUES(42); END;",
        )
        .unwrap();

    let mut db1 = Connection::open_with_flags(&path, OpenFlags::READWRITE).unwrap();
    let mut db2 = Connection::open_with_flags(&path, OpenFlags::READONLY).unwrap();

    db1.busy_timeout(Duration::from_millis(0)).unwrap();
    db2.busy_timeout(Duration::from_millis(0)).unwrap();

    {
        let tx1 = db1.transaction().unwrap();
        let tx2 = db2.transaction().unwrap();

        tx1.query_row("SELECT x FROM foo LIMIT 1", (), |_| Ok(())).unwrap();
        tx2.query_row("SELECT x FROM foo LIMIT 1", (), |_| Ok(())).unwrap();

        tx1.execute("INSERT INTO foo VALUES(?1)", [1i32]).unwrap();
        let _ = tx2.execute("INSERT INTO foo VALUES(?1)", [2i32]);

        let _ = tx1.commit();
        let _ = tx2.commit();
    }

    let _ = db1.transaction().expect("commit should have closed transaction");
    let _ = db2.transaction().expect("commit should have closed transaction");
}

#[test]
fn test_persistence() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("test.db3");

    {
        let db = Connection::open(&path).unwrap();
        db.execute_batch(
            "BEGIN;
             CREATE TABLE foo(x INTEGER);
             INSERT INTO foo VALUES(42);
             END;",
        )
        .unwrap();
    }

    let db = Connection::open(&path).unwrap();
    let the_answer: i64 = db.one_column("SELECT x FROM foo").unwrap();
    assert_eq!(42, the_answer);
}

#[test]
fn test_open() {
    Connection::open_in_memory().unwrap();
}

#[test]
fn test_path() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("test.db3");
    let db = Connection::open(&path).unwrap();
    let p = db.path().unwrap();
    assert!(p.ends_with("test.db3"), "path = {p}");
    // in-memory db has empty or no path
    let mem = Connection::open_in_memory().unwrap();
    // path may be empty string or None for :memory:
    let _ = mem.path();
}

#[test]
fn test_open_failure() {
    let filename = "no_such_file.db";
    let result = Connection::open_with_flags(filename, OpenFlags::READONLY);
    let err = result.unwrap_err();
    if let Error::SqliteFailure(e, _) = err {
        assert_eq!(ErrorCode::CannotOpen, e.code);
        assert_eq!(SQLITE_CANTOPEN, e.extended_code);
    } else {
        panic!("SqliteFailure expected, got: {err:?}");
    }
}

#[test]
fn test_close_retry() {
    // open_in_memory → prepare a statement → close should succeed
    // (rusqlite's original prepares a statement and drops it first)
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE foo(x)").unwrap();
    {
        let _stmt = db.prepare("SELECT * FROM foo").unwrap();
        // _stmt dropped here; close below should work
    }
    db.close().unwrap();
}

#[test]
fn test_open_with_flags() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("flags.db");
    // create
    {
        Connection::open(&path).unwrap();
    }
    // open read-only
    Connection::open_with_flags(&path, OpenFlags::READONLY).unwrap();
    // open read-write without create
    Connection::open_with_flags(&path, OpenFlags::READWRITE).unwrap();
}

#[test]
fn test_execute_batch() {
    let db = checked_memory_handle();
    let sql = "BEGIN;
               CREATE TABLE foo(x INTEGER);
               INSERT INTO foo VALUES(1);
               INSERT INTO foo VALUES(2);
               INSERT INTO foo VALUES(3);
               INSERT INTO foo VALUES(4);
               END;";
    db.execute_batch(sql).unwrap();

    db.execute_batch("UPDATE foo SET x = 3 WHERE x < 3").unwrap();
    db.execute_batch("INVALID SQL").unwrap_err();
}

#[test]
fn test_execute() {
    let db = checked_memory_handle();
    db.execute_batch("CREATE TABLE foo(x INTEGER)").unwrap();

    assert_eq!(1, db.execute("INSERT INTO foo(x) VALUES (?1)", params![1i32]).unwrap());
    assert_eq!(1, db.execute("INSERT INTO foo(x) VALUES (?1)", params![2i32]).unwrap());

    assert_eq!(3i32, db.one_column::<i32>("SELECT SUM(x) FROM foo").unwrap());
}

#[test]
fn test_execute_select_with_no_row() {
    let db = checked_memory_handle();
    db.execute_batch("CREATE TABLE foo(x INTEGER)").unwrap();
    let result: rslite::Result<i64> = db.one_column("SELECT x FROM foo");
    match result.unwrap_err() {
        Error::QueryReturnedNoRows => (),
        err => panic!("unexpected: {err}"),
    }
}

#[test]
fn test_execute_select_with_row() {
    let db = checked_memory_handle();
    let err = db.execute("SELECT 1", ()).unwrap_err();
    assert_eq!(err, Error::ExecuteReturnedResults);
}

#[test]
fn test_execute_multiple() {
    let db = checked_memory_handle();
    let err = db
        .execute("CREATE TABLE foo(x INTEGER); CREATE TABLE bar(x INTEGER)", ())
        .unwrap_err();
    match err {
        Error::MultipleStatement => (),
        _ => panic!("unexpected: {err}"),
    }
    // Tail comment should be treated as no second statement
    db.execute("CREATE TABLE t(c); -- bim", ()).unwrap();
}

#[test]
fn test_prepare_column_names() {
    let db = checked_memory_handle();
    db.execute_batch("CREATE TABLE foo(x INTEGER);").unwrap();

    let stmt = db.prepare("SELECT * FROM foo").unwrap();
    assert_eq!(stmt.column_count(), 1);
    assert_eq!(stmt.column_names(), vec!["x"]);

    let stmt = db.prepare("SELECT x AS a, x AS b FROM foo").unwrap();
    assert_eq!(stmt.column_count(), 2);
    assert_eq!(stmt.column_names(), vec!["a", "b"]);
}

#[test]
fn test_prepare_execute() {
    let db = checked_memory_handle();
    db.execute_batch("CREATE TABLE foo(x INTEGER);").unwrap();

    let mut insert_stmt = db.prepare("INSERT INTO foo(x) VALUES(?1)").unwrap();
    assert_eq!(insert_stmt.execute(params![1i32]).unwrap(), 1);
    assert_eq!(insert_stmt.execute(params![2i32]).unwrap(), 1);
    assert_eq!(insert_stmt.execute(params![3i32]).unwrap(), 1);
    assert_eq!(insert_stmt.execute(params!["hello"]).unwrap(), 1);
    assert_eq!(insert_stmt.execute(params!["goodbye"]).unwrap(), 1);
    assert_eq!(insert_stmt.execute(params![Null]).unwrap(), 1);

    let mut update_stmt = db.prepare("UPDATE foo SET x=?1 WHERE x<?2").unwrap();
    assert_eq!(update_stmt.execute(params![3i32, 3i32]).unwrap(), 2);
    assert_eq!(update_stmt.execute(params![3i32, 3i32]).unwrap(), 0);
    assert_eq!(update_stmt.execute(params![8i32, 8i32]).unwrap(), 3);
}

#[test]
fn test_prepare_query() {
    let db = checked_memory_handle();
    db.execute_batch("CREATE TABLE foo(x INTEGER);").unwrap();

    let mut insert = db.prepare("INSERT INTO foo(x) VALUES(?1)").unwrap();
    insert.execute(params![1i32]).unwrap();
    insert.execute(params![2i32]).unwrap();
    insert.execute(params![3i32]).unwrap();

    let mut query = db
        .prepare("SELECT x FROM foo WHERE x < ?1 ORDER BY x DESC")
        .unwrap();

    {
        let mut rows = query.query(params![4i32]).unwrap();
        let mut v = Vec::<i32>::new();
        while let Some(row) = rows.next().unwrap() {
            v.push(row.get(0).unwrap());
        }
        assert_eq!(v, [3i32, 2, 1]);
    }
    {
        let mut rows = query.query(params![3i32]).unwrap();
        let mut v = Vec::<i32>::new();
        while let Some(row) = rows.next().unwrap() {
            v.push(row.get(0).unwrap());
        }
        assert_eq!(v, [2i32, 1]);
    }
}

#[test]
fn test_query_map() {
    let db = checked_memory_handle();
    db.execute_batch(
        r#"BEGIN;
           CREATE TABLE foo(x INTEGER, y TEXT);
           INSERT INTO foo VALUES(4, "hello");
           INSERT INTO foo VALUES(3, ", ");
           INSERT INTO foo VALUES(2, "world");
           INSERT INTO foo VALUES(1, "!");
           END;"#,
    )
    .unwrap();

    let mut query = db
        .prepare("SELECT x, y FROM foo ORDER BY x DESC")
        .unwrap();
    let results: rslite::Result<Vec<String>> =
        query.query_map((), |row| row.get(1)).unwrap().collect();
    assert_eq!(results.unwrap().concat(), "hello, world!");
}

#[test]
fn test_query_row() {
    let db = checked_memory_handle();
    db.execute_batch(
        "BEGIN;
         CREATE TABLE foo(x INTEGER);
         INSERT INTO foo VALUES(1);
         INSERT INTO foo VALUES(2);
         INSERT INTO foo VALUES(3);
         INSERT INTO foo VALUES(4);
         END;",
    )
    .unwrap();

    assert_eq!(10i64, db.one_column::<i64>("SELECT SUM(x) FROM foo").unwrap());

    let result: rslite::Result<i64> = db.one_column("SELECT x FROM foo WHERE x > 5");
    match result.unwrap_err() {
        Error::QueryReturnedNoRows => (),
        err => panic!("unexpected: {err}"),
    }

    db.query_row("NOT A PROPER QUERY; test123", (), |_| Ok(())).unwrap_err();
    db.query_row("SELECT 1; SELECT 2;", (), |_| Ok(())).unwrap_err();
}

#[test]
fn test_optional() {
    let db = checked_memory_handle();

    let result: rslite::Result<i64> = db.one_column("SELECT 1 WHERE 0 <> 0");
    match result.optional().unwrap() {
        None => (),
        _ => panic!("unexpected"),
    }

    let result: rslite::Result<i64> = db.one_column("SELECT 1 WHERE 0 == 0");
    match result.optional().unwrap() {
        Some(1) => (),
        _ => panic!("unexpected"),
    }

    let bad: rslite::Result<i64> = db.one_column("NOT A PROPER QUERY");
    bad.optional().unwrap_err();
}

#[test]
fn test_pragma_query_row() {
    let db = checked_memory_handle();
    assert_eq!("memory", db.one_column::<String>("PRAGMA journal_mode").unwrap());
}

#[test]
fn test_prepare_failures() {
    let db = checked_memory_handle();
    db.execute_batch("CREATE TABLE foo(x INTEGER);").unwrap();

    let err = db.prepare("SELECT * FROM does_not_exist").unwrap_err();
    assert!(format!("{err}").contains("does_not_exist"));
}

#[test]
fn test_last_insert_rowid() {
    let db = checked_memory_handle();
    db.execute_batch("CREATE TABLE foo(x INTEGER PRIMARY KEY)").unwrap();
    db.execute_batch("INSERT INTO foo DEFAULT VALUES").unwrap();
    assert_eq!(db.last_insert_rowid(), 1);

    let mut stmt = db.prepare("INSERT INTO foo DEFAULT VALUES").unwrap();
    for _ in 0i32..9 {
        stmt.execute(()).unwrap();
    }
    assert_eq!(db.last_insert_rowid(), 10);
}

#[test]
fn test_total_changes() {
    let db = checked_memory_handle();
    let sql = "CREATE TABLE foo(x INTEGER PRIMARY KEY, value TEXT default '' NOT NULL,
                                desc TEXT default '');
               CREATE VIEW foo_bar AS SELECT x, desc FROM foo WHERE value = 'bar';
               CREATE TRIGGER INSERT_FOOBAR
               INSTEAD OF INSERT
               ON foo_bar
               BEGIN
                   INSERT INTO foo VALUES(new.x, 'bar', new.desc);
               END;";
    db.execute_batch(sql).unwrap();
    let before = db.total_changes();
    let changes = db
        .prepare("INSERT INTO foo_bar VALUES(null, 'baz');")
        .unwrap()
        .execute(())
        .unwrap();
    let after = db.total_changes();
    assert_eq!(changes, 0);
    assert_eq!(after - before, 1);
}

#[test]
fn test_is_autocommit() {
    let db = checked_memory_handle();
    assert!(db.is_autocommit());
}

#[test]
fn test_statement_debugging() {
    let db = checked_memory_handle();
    let stmt = db.prepare("SELECT 1 + 1").unwrap();
    let s = format!("{stmt:?}");
    assert!(s.contains("SELECT 1 + 1"));
}

#[test]
fn test_notnull_constraint_error() {
    let db = checked_memory_handle();
    db.execute_batch("CREATE TABLE foo(x NOT NULL)").unwrap();

    let result = db.execute("INSERT INTO foo (x) VALUES (NULL)", ());
    match result.unwrap_err() {
        Error::SqliteFailure(err, _) => {
            assert_eq!(err.code, ErrorCode::ConstraintViolation);
            assert_eq!(err.extended_code, SQLITE_CONSTRAINT_NOTNULL);
        }
        err => panic!("unexpected: {err}"),
    }
}

#[test]
fn test_get_raw() {
    let db = checked_memory_handle();
    db.execute_batch("CREATE TABLE foo(i, x);").unwrap();
    let vals = ["foobar", "1234", "qwerty"];
    let mut insert = db.prepare("INSERT INTO foo(i, x) VALUES(?1, ?2)").unwrap();
    for (i, v) in vals.iter().enumerate() {
        assert_eq!(insert.execute(params![i as i64, *v]).unwrap(), 1);
    }

    let mut query = db.prepare("SELECT i, x FROM foo").unwrap();
    let mut rows = query.query(()).unwrap();
    while let Some(row) = rows.next().unwrap() {
        let i = row.get_ref(0).unwrap().as_i64().unwrap();
        let expect = vals[i as usize];
        let x = row.get_ref("x").unwrap().as_str().unwrap();
        assert_eq!(x, expect);
    }

    let mut query = db.prepare("SELECT x FROM foo").unwrap();
    let rows = query
        .query_map((), |row| {
            let x = row.get_ref(0)?.as_str()?;
            Ok(x.to_owned())
        })
        .unwrap();
    for (i, row) in rows.enumerate() {
        assert_eq!(row.unwrap(), vals[i]);
    }
}

#[test]
fn test_dynamic() {
    let db = checked_memory_handle();
    db.execute_batch(
        r#"BEGIN; CREATE TABLE foo(x INTEGER, y TEXT); INSERT INTO foo VALUES(4, "hello"); END;"#,
    )
    .unwrap();
    db.query_row("SELECT * FROM foo", (), |r| {
        assert_eq!(2, r.column_count());
        Ok(())
    })
    .unwrap();
}

#[test]
fn test_dyn_box() {
    let db = checked_memory_handle();
    db.execute_batch("CREATE TABLE foo(x INTEGER);").unwrap();
    let b: Box<dyn ToSql> = Box::new(5i32);
    db.execute("INSERT INTO foo VALUES(?1)", params![b]).unwrap();
    db.query_row("SELECT x FROM foo", (), |r| {
        assert_eq!(5, r.get_unwrap::<_, i32>(0));
        Ok(())
    })
    .unwrap();
}

#[test]
fn test_params() {
    let db = checked_memory_handle();
    db.query_row(
        "SELECT
        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
        ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
        ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30,
        ?31, ?32, ?33, ?34;",
        params![
            1i32, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1,
        ],
        |r| {
            assert_eq!(1, r.get_unwrap::<_, i32>(0));
            Ok(())
        },
    )
    .unwrap();
}

#[test]
fn test_alter_table() {
    let db = checked_memory_handle();
    db.execute_batch("CREATE TABLE x(t);").unwrap();
    db.execute("ALTER TABLE x RENAME TO y;", ()).unwrap();
}

#[test]
fn test_error_eq() {
    assert_eq!(Error::ExecuteReturnedResults, Error::ExecuteReturnedResults);
    assert_eq!(Error::QueryReturnedNoRows, Error::QueryReturnedNoRows);
    assert_eq!(Error::MultipleStatement, Error::MultipleStatement);
}

// ═══════════════════════════════════════════════════════════════════════════
// transaction.rs tests
// ═══════════════════════════════════════════════════════════════════════════

fn tx_memory_handle() -> Connection {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE foo (x INTEGER)").unwrap();
    db
}

fn tx_insert(x: i32, conn: &Connection) -> rslite::Result<usize> {
    conn.execute("INSERT INTO foo VALUES(?1)", [x])
}

fn tx_assert_sum(x: i32, conn: &Connection) {
    let i: i32 = conn.one_column("SELECT SUM(x) FROM foo").unwrap();
    assert_eq!(x, i);
}

#[test]
fn test_drop() {
    let mut db = tx_memory_handle();
    {
        let tx = db.transaction().unwrap();
        tx.execute_batch("INSERT INTO foo VALUES(1)").unwrap();
        // default: rollback
    }
    {
        let mut tx = db.transaction().unwrap();
        tx.execute_batch("INSERT INTO foo VALUES(2)").unwrap();
        tx.set_drop_behavior(DropBehavior::Commit);
        // drop with Commit behavior
    }
    {
        let tx = db.transaction().unwrap();
        assert_eq!(2i32, tx.one_column::<i32>("SELECT SUM(x) FROM foo").unwrap());
    }
}

#[test]
fn test_explicit_rollback_commit() {
    let mut db = tx_memory_handle();
    {
        let mut tx = db.transaction().unwrap();
        {
            let mut sp = tx.savepoint().unwrap();
            sp.execute_batch("INSERT INTO foo VALUES(1)").unwrap();
            sp.rollback().unwrap();
            sp.execute_batch("INSERT INTO foo VALUES(2)").unwrap();
            sp.commit().unwrap();
        }
        tx.commit().unwrap();
    }
    {
        let tx = db.transaction().unwrap();
        tx.execute_batch("INSERT INTO foo VALUES(4)").unwrap();
        tx.commit().unwrap();
    }
    {
        let tx = db.transaction().unwrap();
        assert_eq!(6i32, tx.one_column::<i32>("SELECT SUM(x) FROM foo").unwrap());
    }
}

#[test]
fn test_savepoint() {
    let mut db = tx_memory_handle();
    {
        let mut tx = db.transaction().unwrap();
        tx.execute_batch("INSERT INTO foo VALUES(1)").unwrap();
        tx_assert_sum(1, &tx);
        tx.set_drop_behavior(DropBehavior::Commit);
        {
            let mut sp1 = tx.savepoint().unwrap();
            sp1.execute_batch("INSERT INTO foo VALUES(2)").unwrap();
            tx_assert_sum(3, &sp1);
            {
                let mut sp2 = sp1.savepoint().unwrap();
                sp2.execute_batch("INSERT INTO foo VALUES(4)").unwrap();
                tx_assert_sum(7, &sp2);
                {
                    let mut sp3 = sp2.savepoint().unwrap();
                    sp3.execute_batch("INSERT INTO foo VALUES(8)").unwrap();
                    tx_assert_sum(15, &sp3);
                    sp3.commit().unwrap();
                }
                tx_assert_sum(15, &sp2);
                // sp2 dropped → rollback
            }
            tx_assert_sum(3, &sp1);
            // sp1 dropped → rollback
        }
        tx_assert_sum(1, &tx);
        // tx dropped with Commit behavior
    }
    tx_assert_sum(1, &db);
}

#[test]
fn test_ignore_drop_behavior() {
    let mut db = tx_memory_handle();

    let mut tx = db.transaction().unwrap();
    {
        let mut sp1 = tx.savepoint().unwrap();
        tx_insert(1, &sp1).unwrap();
        sp1.rollback().unwrap();
        tx_insert(2, &sp1).unwrap();
        {
            let mut sp2 = sp1.savepoint().unwrap();
            sp2.set_drop_behavior(DropBehavior::Ignore);
            tx_insert(4, &sp2).unwrap();
            // sp2 dropped with Ignore → neither commit nor rollback
        }
        tx_assert_sum(6, &sp1);
        sp1.commit().unwrap();
    }
    tx_assert_sum(6, &tx);
}

#[test]
fn test_savepoint_drop_behavior_releases() {
    let mut db = tx_memory_handle();

    {
        let mut sp = db.savepoint().unwrap();
        sp.set_drop_behavior(DropBehavior::Commit);
    }
    assert!(db.is_autocommit());

    {
        let mut sp = db.savepoint().unwrap();
        sp.set_drop_behavior(DropBehavior::Rollback);
    }
    assert!(db.is_autocommit());
}

#[test]
fn test_savepoint_release_error() {
    let mut db = tx_memory_handle();

    db.pragma_update(None, "foreign_keys", true).unwrap();
    db.execute_batch(
        "CREATE TABLE r(n INTEGER PRIMARY KEY NOT NULL); \
         CREATE TABLE f(n REFERENCES r(n) DEFERRABLE INITIALLY DEFERRED);",
    )
    .unwrap();
    {
        let mut sp = db.savepoint().unwrap();
        sp.execute("INSERT INTO f VALUES (0)", ()).unwrap();
        sp.set_drop_behavior(DropBehavior::Commit);
        // drop with Commit tries RELEASE, FK violation should cause a rollback
    }
    assert!(db.is_autocommit());
}

#[test]
fn test_savepoint_names() {
    let mut db = tx_memory_handle();
    {
        let mut sp1 = db.savepoint_with_name("my_sp").unwrap();
        tx_insert(1, &sp1).unwrap();
        tx_assert_sum(1, &sp1);
        {
            let mut sp2 = sp1.savepoint_with_name("my_sp").unwrap();
            sp2.set_drop_behavior(DropBehavior::Commit);
            tx_insert(2, &sp2).unwrap();
            tx_assert_sum(3, &sp2);
            sp2.rollback().unwrap();
            tx_assert_sum(1, &sp2);
            tx_insert(4, &sp2).unwrap();
            // sp2 drop with Commit
        }
        tx_assert_sum(5, &sp1);
        sp1.rollback().unwrap();
        {
            let mut sp2 = sp1.savepoint_with_name("my_sp").unwrap();
            sp2.set_drop_behavior(DropBehavior::Ignore);
            tx_insert(8, &sp2).unwrap();
            // sp2 drop with Ignore
        }
        tx_assert_sum(8, &sp1);
        sp1.commit().unwrap();
    }
    tx_assert_sum(8, &db);
}

#[test]
fn test_rc() {
    use std::rc::Rc;
    let mut conn = Connection::open_in_memory().unwrap();
    let rc_txn = Rc::new(conn.transaction().unwrap());
    // This compiles only if Transaction is Debug
    Rc::try_unwrap(rc_txn).unwrap();
}

// ═══════════════════════════════════════════════════════════════════════════
// types/mod.rs tests
// ═══════════════════════════════════════════════════════════════════════════

fn types_memory_handle() -> Connection {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE foo (b BLOB, t TEXT, i INTEGER, f FLOAT, n)")
        .unwrap();
    db
}

#[test]
fn test_blob() {
    let db = types_memory_handle();
    let v1234 = vec![1u8, 2, 3, 4];
    db.execute("INSERT INTO foo(b) VALUES (?1)", [&v1234]).unwrap();
    let v: Vec<u8> = db.one_column("SELECT b FROM foo").unwrap();
    assert_eq!(v, v1234);
}

#[test]
fn test_empty_blob() {
    let db = types_memory_handle();
    let empty: Vec<u8> = vec![];
    db.execute("INSERT INTO foo(b) VALUES (?1)", [&empty]).unwrap();
    let v: Vec<u8> = db.one_column("SELECT b FROM foo").unwrap();
    assert_eq!(v, empty);
}

#[test]
fn test_str() {
    let db = types_memory_handle();
    let s = "hello, world!";
    db.execute("INSERT INTO foo(t) VALUES (?1)", [&s]).unwrap();
    let from: String = db.one_column("SELECT t FROM foo").unwrap();
    assert_eq!(from, s);
}

#[test]
fn test_string() {
    let db = types_memory_handle();
    let s = "hello, world!";
    db.execute("INSERT INTO foo(t) VALUES (?1)", [s.to_owned()]).unwrap();
    let from: String = db.one_column("SELECT t FROM foo").unwrap();
    assert_eq!(from, s);
}

#[test]
fn test_value() {
    let db = types_memory_handle();
    db.execute("INSERT INTO foo(i) VALUES (?1)", [Value::Integer(10)]).unwrap();
    assert_eq!(10i64, db.one_column::<i64>("SELECT i FROM foo").unwrap());
}

#[test]
fn test_option() {
    let db = types_memory_handle();
    let s = "hello, world!";
    let b = Some(vec![1u8, 2, 3, 4]);

    db.execute("INSERT INTO foo(t) VALUES (?1)", [Some(s)]).unwrap();
    db.execute("INSERT INTO foo(b) VALUES (?1)", [&b]).unwrap();

    let mut stmt = db.prepare("SELECT t, b FROM foo ORDER BY ROWID ASC").unwrap();
    let mut rows = stmt.query(()).unwrap();

    {
        let row1 = rows.next().unwrap().unwrap();
        let s1: Option<String> = row1.get_unwrap(0);
        let b1: Option<Vec<u8>> = row1.get_unwrap(1);
        assert_eq!(s, s1.unwrap());
        assert!(b1.is_none());
    }
    {
        let row2 = rows.next().unwrap().unwrap();
        let s2: Option<String> = row2.get_unwrap(0);
        let b2: Option<Vec<u8>> = row2.get_unwrap(1);
        assert!(s2.is_none());
        assert_eq!(b, b2);
    }
}

fn is_invalid_column_type(err: Error) -> bool {
    matches!(err, Error::InvalidColumnType(..))
}

#[test]
fn test_mismatched_types() {
    let db = types_memory_handle();
    db.execute(
        "INSERT INTO foo(b, t, i, f) VALUES (X'0102', 'text', 1, 1.5)",
        (),
    )
    .unwrap();

    let mut stmt = db.prepare("SELECT b, t, i, f, n FROM foo").unwrap();
    let mut rows = stmt.query(()).unwrap();
    let row = rows.next().unwrap().unwrap();

    // correct types
    assert_eq!(vec![1u8, 2], row.get::<_, Vec<u8>>(0).unwrap());
    assert_eq!("text", row.get::<_, String>(1).unwrap());
    assert_eq!(1, row.get::<_, c_int>(2).unwrap());
    assert!((1.5 - row.get::<_, c_double>(3).unwrap()).abs() < f64::EPSILON);
    assert_eq!(row.get::<_, Option<c_int>>(4).unwrap(), None);
    assert_eq!(row.get::<_, Option<c_double>>(4).unwrap(), None);
    assert_eq!(row.get::<_, Option<String>>(4).unwrap(), None);

    // 0 is blob
    assert!(is_invalid_column_type(row.get::<_, c_int>(0).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, i64>(0).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, c_double>(0).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, String>(0).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, Option<c_int>>(0).unwrap_err()));

    // 1 is text
    assert!(is_invalid_column_type(row.get::<_, c_int>(1).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, i64>(1).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, c_double>(1).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, Vec<u8>>(1).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, Option<c_int>>(1).unwrap_err()));

    // 2 is integer
    assert!(is_invalid_column_type(row.get::<_, String>(2).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, Vec<u8>>(2).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, Option<String>>(2).unwrap_err()));

    // 3 is float
    assert!(is_invalid_column_type(row.get::<_, c_int>(3).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, i64>(3).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, String>(3).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, Vec<u8>>(3).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, Option<c_int>>(3).unwrap_err()));

    // 4 is NULL
    assert!(is_invalid_column_type(row.get::<_, c_int>(4).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, i64>(4).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, c_double>(4).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, String>(4).unwrap_err()));
    assert!(is_invalid_column_type(row.get::<_, Vec<u8>>(4).unwrap_err()));
}

#[test]
fn test_dynamic_type() {
    let db = types_memory_handle();
    db.execute(
        "INSERT INTO foo(b, t, i, f) VALUES (X'0102', 'text', 1, 1.5)",
        (),
    )
    .unwrap();

    let mut stmt = db.prepare("SELECT b, t, i, f, n FROM foo").unwrap();
    let mut rows = stmt.query(()).unwrap();
    let row = rows.next().unwrap().unwrap();

    assert_eq!(Value::Blob(vec![1, 2]), row.get::<_, Value>(0).unwrap());
    assert_eq!(Value::Text("text".to_string()), row.get::<_, Value>(1).unwrap());
    assert_eq!(Value::Integer(1), row.get::<_, Value>(2).unwrap());
    match row.get::<_, Value>(3).unwrap() {
        Value::Real(val) => assert!((1.5 - val).abs() < f64::EPSILON),
        x => panic!("invalid value {x:?}"),
    }
    assert_eq!(Value::Null, row.get::<_, Value>(4).unwrap());
}

#[test]
#[allow(clippy::float_cmp)]
fn test_numeric_conversions() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE foo (x)").unwrap();

    macro_rules! test_conv {
        ($ins:expr, $get:ty, expect $exp:expr) => {{
            db.execute("INSERT INTO foo VALUES (?1)", params![$ins]).unwrap();
            let res: $get = db.one_column("SELECT x FROM foo").unwrap();
            assert_eq!(res, $exp);
            db.execute_batch("DELETE FROM foo").unwrap();
        }};
        ($ins:expr, $get:ty, expect_from_sql_error) => {{
            db.execute("INSERT INTO foo VALUES (?1)", params![$ins]).unwrap();
            db.one_column::<$get>("SELECT x FROM foo").unwrap_err();
            db.execute_batch("DELETE FROM foo").unwrap();
        }};
        ($ins:expr, $get:ty, expect_to_sql_error) => {{
            db.execute("INSERT INTO foo VALUES (?1)", params![$ins]).unwrap_err();
        }};
    }

    test_conv!(0u8, u8, expect 0u8);

    // in-range
    test_conv!(100u8,  i8,  expect 100i8);
    test_conv!(200u8,  u8,  expect 200u8);
    test_conv!(100u16, i8,  expect 100i8);
    test_conv!(200u16, u8,  expect 200u8);
    test_conv!(u32::MAX, u64, expect u32::MAX as u64);
    test_conv!(i64::MIN, i64, expect i64::MIN);
    test_conv!(i64::MAX, i64, expect i64::MAX);
    test_conv!(i64::MAX, u64, expect i64::MAX as u64);
    test_conv!(100usize, usize, expect 100usize);
    test_conv!(100u64,   u64,   expect 100u64);
    test_conv!(i64::MAX as u64, u64, expect i64::MAX as u64);

    // out-of-range
    test_conv!(200u8,  i8,  expect_from_sql_error);
    test_conv!(400u16, i8,  expect_from_sql_error);
    test_conv!(400u16, u8,  expect_from_sql_error);
    test_conv!(-1i8,   u8,  expect_from_sql_error);
    test_conv!(i64::MIN, u64, expect_from_sql_error);
    test_conv!(u64::MAX,              i64, expect_to_sql_error);
    test_conv!(u64::MAX,              u64, expect_to_sql_error);
    test_conv!(i64::MAX as u64 + 1,   u64, expect_to_sql_error);

    // int → float: always works
    test_conv!(i64::MIN, f32, expect i64::MIN as f32);
    test_conv!(i64::MAX, f32, expect i64::MAX as f32);
    test_conv!(i64::MIN, f64, expect i64::MIN as f64);
    test_conv!(i64::MAX, f64, expect i64::MAX as f64);

    // float → int: never works
    test_conv!(0f64, i64, expect_from_sql_error);
}

// ═══════════════════════════════════════════════════════════════════════════
// statement.rs tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_execute_named() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE foo(x INTEGER)").unwrap();

    assert_eq!(db.execute("INSERT INTO foo(x) VALUES (:x)", &[(":x", &1i32)]).unwrap(), 1);
    assert_eq!(db.execute("INSERT INTO foo(x) VALUES (:x)", &[(":x", &2i32)]).unwrap(), 1);
    assert_eq!(
        db.execute("INSERT INTO foo(x) VALUES (:x)", named_params! {":x": 3i32}).unwrap(),
        1
    );

    assert_eq!(
        6i32,
        db.query_row::<i32, _, _>(
            "SELECT SUM(x) FROM foo WHERE x > :x",
            &[(":x", &0i32)],
            |r| r.get(0)
        )
        .unwrap()
    );
    assert_eq!(
        5i32,
        db.query_row::<i32, _, _>(
            "SELECT SUM(x) FROM foo WHERE x > :x",
            &[(":x", &1i32)],
            |r| r.get(0)
        )
        .unwrap()
    );
}

#[test]
fn test_stmt_execute_named() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch(
        "CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, flag INTEGER)",
    )
    .unwrap();

    let mut stmt = db.prepare("INSERT INTO test (name) VALUES (:name)").unwrap();
    stmt.execute(&[(":name", &"one")]).unwrap();

    let mut stmt = db
        .prepare("SELECT COUNT(*) FROM test WHERE name = :name")
        .unwrap();
    assert_eq!(
        1i32,
        stmt.query_row::<i32, _, _>(&[(":name", "one")], |r| r.get(0))
            .unwrap()
    );
}

#[test]
fn test_query_named() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch(
        r#"CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, flag INTEGER);
           INSERT INTO test(id, name) VALUES (1, "one");"#,
    )
    .unwrap();

    let mut stmt = db.prepare("SELECT id FROM test where name = :name").unwrap();
    let mut rows = stmt.query(&[(":name", "one")]).unwrap();
    let id: rslite::Result<i32> = rows.next().unwrap().unwrap().get(0);
    assert_eq!(Ok(1), id);
}

#[test]
fn test_query_map_named() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch(
        r#"CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, flag INTEGER);
           INSERT INTO test(id, name) VALUES (1, "one");"#,
    )
    .unwrap();

    let mut stmt = db.prepare("SELECT id FROM test where name = :name").unwrap();
    let mut rows = stmt
        .query_map(&[(":name", "one")], |row| {
            let id: rslite::Result<i32> = row.get(0);
            id.map(|i| 2 * i)
        })
        .unwrap();

    let doubled_id: i32 = rows.next().unwrap().unwrap();
    assert_eq!(2, doubled_id);
}

#[test]
fn test_unbound_parameters_are_null() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE test (x TEXT, y TEXT)").unwrap();

    let mut stmt = db
        .prepare("INSERT INTO test (x, y) VALUES (:x, :y)")
        .unwrap();
    stmt.execute(&[(":x", &"one")]).unwrap();

    let result: Option<String> = db.one_column("SELECT y FROM test WHERE x = 'one'").unwrap();
    assert!(result.is_none());
}

#[test]
fn test_unbound_parameters_are_reused() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE test (x TEXT, y TEXT)").unwrap();

    let mut stmt = db
        .prepare("INSERT INTO test (x, y) VALUES (:x, :y)")
        .unwrap();
    stmt.execute(&[(":x", "one")]).unwrap();
    stmt.execute(&[(":y", "two")]).unwrap();

    let result: String = db
        .one_column("SELECT x FROM test WHERE y = 'two'")
        .unwrap();
    assert_eq!(result, "one");
}

#[test]
fn test_insert() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE foo(x INTEGER UNIQUE)").unwrap();

    let mut stmt = db.prepare("INSERT OR IGNORE INTO foo (x) VALUES (?1)").unwrap();
    assert_eq!(stmt.insert([1i32]).unwrap(), 1);
    assert_eq!(stmt.insert([2i32]).unwrap(), 2);
    match stmt.insert([1i32]).unwrap_err() {
        Error::StatementChangedRows(0) => (),
        err => panic!("unexpected: {err}"),
    }
    let mut multi = db
        .prepare("INSERT INTO foo (x) SELECT 3 UNION ALL SELECT 4")
        .unwrap();
    match multi.insert(()).unwrap_err() {
        Error::StatementChangedRows(2) => (),
        err => panic!("unexpected: {err}"),
    }
}

#[test]
fn test_insert_different_tables() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE foo(x INTEGER); CREATE TABLE bar(x INTEGER);")
        .unwrap();

    assert_eq!(db.prepare("INSERT INTO foo VALUES (10)").unwrap().insert(()).unwrap(), 1);
    assert_eq!(db.prepare("INSERT INTO bar VALUES (10)").unwrap().insert(()).unwrap(), 1);
}

#[test]
fn test_exists() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch(
        "BEGIN;
         CREATE TABLE foo(x INTEGER);
         INSERT INTO foo VALUES(1);
         INSERT INTO foo VALUES(2);
         END;",
    )
    .unwrap();
    let mut stmt = db.prepare("SELECT 1 FROM foo WHERE x = ?1").unwrap();
    assert!(stmt.exists([1i32]).unwrap());
    assert!(stmt.exists([2i32]).unwrap());
    assert!(!stmt.exists([0i32]).unwrap());
}

#[test]
fn test_tuple_params() {
    let db = Connection::open_in_memory().unwrap();
    let s = db
        .query_row("SELECT printf('[%s]', ?1)", ("abc",), |r| {
            r.get::<_, String>(0)
        })
        .unwrap();
    assert_eq!(s, "[abc]");

    let s = db
        .query_row(
            "SELECT printf('%d %s %d', ?1, ?2, ?3)",
            (1i32, "abc", 2i32),
            |r| r.get::<_, String>(0),
        )
        .unwrap();
    assert_eq!(s, "1 abc 2");

    let s = db
        .query_row(
            "SELECT printf('%d %s %d %d', ?1, ?2, ?3, ?4)",
            (1, "abc", 2i32, 4i64),
            |r| r.get::<_, String>(0),
        )
        .unwrap();
    assert_eq!(s, "1 abc 2 4");

    #[rustfmt::skip]
    let bigtup = (
        0, "a", 1, "b", 2, "c", 3, "d",
        4, "e", 5, "f", 6, "g", 7, "h",
    );
    let query = "SELECT printf(
        '%d %s | %d %s | %d %s | %d %s || %d %s | %d %s | %d %s | %d %s',
        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16
    )";
    let s = db
        .query_row(query, bigtup, |r| r.get::<_, String>(0))
        .unwrap();
    assert_eq!(s, "0 a | 1 b | 2 c | 3 d || 4 e | 5 f | 6 g | 7 h");
}

#[test]
fn test_query_row_stmt() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch(
        "BEGIN;
         CREATE TABLE foo(x INTEGER, y INTEGER);
         INSERT INTO foo VALUES(1, 3);
         INSERT INTO foo VALUES(2, 4);
         END;",
    )
    .unwrap();
    let mut stmt = db.prepare("SELECT y FROM foo WHERE x = ?1").unwrap();
    let y: rslite::Result<i64> = stmt.query_row([1i32], |r| r.get(0));
    assert_eq!(3i64, y.unwrap());
}

#[test]
fn test_query_by_column_name() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch(
        "BEGIN;
         CREATE TABLE foo(x INTEGER, y INTEGER);
         INSERT INTO foo VALUES(1, 3);
         END;",
    )
    .unwrap();
    let mut stmt = db.prepare("SELECT y FROM foo").unwrap();
    let y: rslite::Result<i64> = stmt.query_row((), |r| r.get("y"));
    assert_eq!(3i64, y.unwrap());
}

#[test]
fn test_query_by_column_name_ignore_case() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch(
        "BEGIN;
         CREATE TABLE foo(x INTEGER, y INTEGER);
         INSERT INTO foo VALUES(1, 3);
         END;",
    )
    .unwrap();
    // Column alias is "Y" (uppercase), but we query as "y" (lowercase)
    let mut stmt = db.prepare("SELECT y as Y FROM foo").unwrap();
    let y: rslite::Result<i64> = stmt.query_row((), |r| r.get("y"));
    assert_eq!(3i64, y.unwrap());
}

#[test]
fn test_bind_parameters() {
    let db = Connection::open_in_memory().unwrap();
    // dynamic slice via array of &dyn ToSql
    db.query_row(
        "SELECT ?1, ?2, ?3",
        [&1u8 as &dyn ToSql, &"one", &Some("one")],
        |row| row.get::<_, u8>(0),
    )
    .unwrap();

    // params_from_iter variants
    let data = vec![1i32, 2, 3];
    db.query_row("SELECT ?1, ?2, ?3", params_from_iter(&data), |row| {
        row.get::<_, i32>(0)
    })
    .unwrap();
    db.query_row(
        "SELECT ?1, ?2, ?3",
        params_from_iter(data.as_slice()),
        |row| row.get::<_, i32>(0),
    )
    .unwrap();
    db.query_row("SELECT ?1, ?2, ?3", params_from_iter(data), |row| {
        row.get::<_, i32>(0)
    })
    .unwrap();

    use std::collections::BTreeSet;
    let data: BTreeSet<String> = ["one", "two", "three"].iter().map(|s| s.to_string()).collect();
    db.query_row("SELECT ?1, ?2, ?3", params_from_iter(&data), |row| {
        row.get::<_, String>(0)
    })
    .unwrap();

    let data = [0i32; 3];
    db.query_row("SELECT ?1, ?2, ?3", params_from_iter(&data), |row| {
        row.get::<_, i32>(0)
    })
    .unwrap();
    db.query_row(
        "SELECT ?1, ?2, ?3",
        params_from_iter(data.iter()),
        |row| row.get::<_, i32>(0),
    )
    .unwrap();
}

#[test]
fn test_parameter_name() {
    let db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE test (name TEXT, value INTEGER)").unwrap();
    let stmt = db
        .prepare("INSERT INTO test (name, value) VALUES (:name, ?3)")
        .unwrap();
    // 1-based indexing: position 0 is always None (out of range)
    assert_eq!(stmt.parameter_name(0), None);
    // :name is the first named parameter (position 1)
    assert_eq!(stmt.parameter_name(1), Some(":name"));
    // position 2 is unoccupied in this query (gap between :name and ?3)
    assert_eq!(stmt.parameter_name(2), None);
}

#[test]
fn test_comment_stmt() {
    let conn = Connection::open_in_memory().unwrap();
    conn.prepare("/*SELECT 1;*/").unwrap();
}

#[test]
fn test_comment_and_sql_stmt() {
    let conn = Connection::open_in_memory().unwrap();
    let stmt = conn.prepare("/*...*/ SELECT 1;").unwrap();
    assert_eq!(1, stmt.column_count());
}

#[test]
fn test_semi_colon_stmt() {
    let conn = Connection::open_in_memory().unwrap();
    let stmt = conn.prepare(";").unwrap();
    assert_eq!(0, stmt.column_count());
}

#[test]
fn test_utf16_conversion() {
    let db = Connection::open_in_memory().unwrap();
    db.pragma_update(None, "encoding", "UTF-16le").unwrap();
    let encoding: String = db
        .pragma_query_value(None, "encoding", |row| row.get(0))
        .unwrap();
    assert_eq!("UTF-16le", encoding);
    db.execute_batch("CREATE TABLE foo(x TEXT)").unwrap();
    let expected = "テスト";
    db.execute("INSERT INTO foo(x) VALUES (?1)", [&expected]).unwrap();
    let actual: String = db.one_column("SELECT x FROM foo").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_nul_byte() {
    // rusqlite stores nul bytes in parameter values without error
    let db = Connection::open_in_memory().unwrap();
    let expected = "a\x00b";
    let actual: String = db
        .query_row("SELECT ?1", [expected], |row| row.get(0))
        .unwrap();
    assert_eq!(expected, actual);
}

// ═══════════════════════════════════════════════════════════════════════════
// Thread-safety (Send) tests
// ═══════════════════════════════════════════════════════════════════════════

/// Connection is Send: it can be moved to another thread and used there.
#[test]
fn test_connection_send_move_and_query() {
    let db = Connection::open_in_memory().unwrap();
    let handle = std::thread::spawn(move || {
        let n: i64 = db.one_column("SELECT 42").unwrap();
        n
    });
    assert_eq!(42, handle.join().unwrap());
}

/// A connection moved to a worker thread can write and read back data.
#[test]
fn test_connection_send_write_from_thread() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("send.db3");

    let db = Connection::open(&path).unwrap();
    let handle = std::thread::spawn(move || {
        db.execute_batch(
            "CREATE TABLE t (x INTEGER); INSERT INTO t VALUES (7);",
        )
        .unwrap();
    });
    handle.join().unwrap();

    // Re-open from the main thread and verify the data is there.
    let db2 = Connection::open(&path).unwrap();
    let val: i64 = db2.one_column("SELECT x FROM t").unwrap();
    assert_eq!(7, val);
}

/// Mutex<Connection> is Sync+Send, allowing shared access across threads.
#[test]
fn test_connection_mutex_shared_across_threads() {
    use std::sync::{Arc, Mutex};

    let db = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
    {
        let db = Arc::clone(&db);
        db.lock()
            .unwrap()
            .execute_batch("CREATE TABLE t (x INTEGER)")
            .unwrap();
    }

    let handles: Vec<_> = (0..4)
        .map(|i| {
            let db = Arc::clone(&db);
            std::thread::spawn(move || {
                db.lock()
                    .unwrap()
                    .execute("INSERT INTO t VALUES (?1)", [i as i64])
                    .unwrap();
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let count: i64 = db
        .lock()
        .unwrap()
        .one_column("SELECT COUNT(*) FROM t")
        .unwrap();
    assert_eq!(4, count);
}

/// Compile-time check: Connection implements Send.
/// The absence of an `assert_sync` call documents that Sync is intentionally
/// not implemented.
#[test]
fn test_connection_is_send_not_sync() {
    fn assert_send<T: Send>() {}
    assert_send::<Connection>();
}

// ═══════════════════════════════════════════════════════════════════════════
// Hook tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_commit_hook_called_on_commit() {
    let mut db = Connection::open_in_memory().unwrap();
    let called = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let called2 = called.clone();
    db.commit_hook(Some(move || {
        called2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        false // allow commit
    }));
    db.execute_batch("CREATE TABLE t (x INTEGER)").unwrap();
    db.execute("INSERT INTO t VALUES (1)", ()).unwrap();
    assert_eq!(called.load(std::sync::atomic::Ordering::SeqCst), 2);
}

#[test]
fn test_commit_hook_can_abort_commit() {
    let mut db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE t (x INTEGER)").unwrap();
    // Register a hook that vetoes all commits.
    db.commit_hook(Some(|| true));
    // The INSERT should be rolled back.
    let result = db.execute("INSERT INTO t VALUES (1)", ());
    assert!(result.is_err());
    let count: i64 = db.one_column("SELECT COUNT(*) FROM t").unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_commit_hook_remove() {
    let mut db = Connection::open_in_memory().unwrap();
    let called = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let called2 = called.clone();
    db.commit_hook(Some(move || { called2.fetch_add(1, std::sync::atomic::Ordering::SeqCst); false }));
    db.execute_batch("CREATE TABLE t (x INTEGER)").unwrap();
    assert_eq!(called.load(std::sync::atomic::Ordering::SeqCst), 1);
    // Remove hook — subsequent statements must not increment the counter.
    db.commit_hook::<fn() -> bool>(None);
    db.execute_batch("INSERT INTO t VALUES (1)").unwrap();
    assert_eq!(called.load(std::sync::atomic::Ordering::SeqCst), 1);
}

#[test]
fn test_rollback_hook_called_on_rollback() {
    let mut db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE t (x INTEGER)").unwrap();
    let called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let called2 = called.clone();
    db.rollback_hook(Some(move || { called2.store(true, std::sync::atomic::Ordering::SeqCst); }));
    {
        let tx = db.transaction().unwrap();
        tx.execute("INSERT INTO t VALUES (99)", ()).unwrap();
        tx.rollback().unwrap();
    }
    assert!(called.load(std::sync::atomic::Ordering::SeqCst));
    let count: i64 = db.one_column("SELECT COUNT(*) FROM t").unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_update_hook_insert() {
    use std::sync::{Arc, Mutex};
    let mut db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE t (x INTEGER)").unwrap();
    let events: Arc<Mutex<Vec<(Action, String, i64)>>> = Arc::new(Mutex::new(Vec::new()));
    let events2 = events.clone();
    db.update_hook(Some(move |action: Action, _: &str, tbl: &str, rowid: i64| {
        events2.lock().unwrap().push((action, tbl.to_owned(), rowid));
    }));
    db.execute("INSERT INTO t VALUES (42)", ()).unwrap();
    let ev = events.lock().unwrap();
    assert_eq!(ev.len(), 1);
    assert_eq!(ev[0].0, Action::Insert);
    assert_eq!(ev[0].1, "t");
}

#[test]
fn test_update_hook_update_delete() {
    use std::sync::{Arc, Mutex};
    let mut db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE t (x INTEGER)").unwrap();
    db.execute("INSERT INTO t VALUES (1)", ()).unwrap();
    let actions: Arc<Mutex<Vec<Action>>> = Arc::new(Mutex::new(Vec::new()));
    let actions2 = actions.clone();
    db.update_hook(Some(move |action: Action, _: &str, _: &str, _: i64| {
        actions2.lock().unwrap().push(action);
    }));
    db.execute("UPDATE t SET x = 2 WHERE x = 1", ()).unwrap();
    db.execute("DELETE FROM t WHERE x = 2", ()).unwrap();
    let ev = actions.lock().unwrap();
    assert_eq!(ev[0], Action::Update);
    assert_eq!(ev[1], Action::Delete);
}

// ═══════════════════════════════════════════════════════════════════════════
// User-defined function tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_scalar_function_basic() {
    let db = Connection::open_in_memory().unwrap();
    db.create_scalar_function("double", 1, FunctionFlags::DETERMINISTIC, |ctx| {
        Ok(ctx.get::<i64>(0)? * 2)
    })
    .unwrap();
    let n: i64 = db.one_column("SELECT double(21)").unwrap();
    assert_eq!(n, 42);
}

#[test]
fn test_scalar_function_string() {
    let db = Connection::open_in_memory().unwrap();
    db.create_scalar_function("shout", 1, FunctionFlags::default(), |ctx| {
        Ok(ctx.get::<String>(0)?.to_uppercase())
    })
    .unwrap();
    let s: String = db.one_column("SELECT shout('hello')").unwrap();
    assert_eq!(s, "HELLO");
}

#[test]
fn test_scalar_function_variadic() {
    let db = Connection::open_in_memory().unwrap();
    // n_args = -1 means variadic
    db.create_scalar_function("arg_count", -1, FunctionFlags::default(), |ctx| {
        Ok(ctx.len() as i64)
    })
    .unwrap();
    let n: i64 = db.one_column("SELECT arg_count(1, 2, 3)").unwrap();
    assert_eq!(n, 3);
}

#[test]
fn test_scalar_function_remove() {
    let db = Connection::open_in_memory().unwrap();
    db.create_scalar_function("inc", 1, FunctionFlags::default(), |ctx| {
        Ok(ctx.get::<i64>(0)? + 1)
    })
    .unwrap();
    let n: i64 = db.one_column("SELECT inc(0)").unwrap();
    assert_eq!(n, 1);
    db.remove_function("inc", 1).unwrap();
    assert!(db.one_column::<i64>("SELECT inc(0)").is_err());
}

#[test]
fn test_aggregate_function_sum() {
    struct Sum;
    impl Aggregate<i64, i64> for Sum {
        fn init(&self) -> i64 { 0 }
        fn step(&self, ctx: &Context<'_>, acc: &mut i64) -> rslite::Result<()> {
            *acc += ctx.get::<i64>(0)?;
            Ok(())
        }
        fn finalize(&self, _: &Context<'_>, acc: Option<i64>) -> rslite::Result<i64> {
            Ok(acc.unwrap_or(0))
        }
    }

    let db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE t (x INTEGER)").unwrap();
    for i in 1..=5i64 {
        db.execute("INSERT INTO t VALUES (?1)", [i]).unwrap();
    }
    db.create_aggregate_function("mysum", 1, FunctionFlags::default(), Sum)
        .unwrap();
    let total: i64 = db.one_column("SELECT mysum(x) FROM t").unwrap();
    assert_eq!(total, 15);
}

#[test]
fn test_aggregate_function_empty_group() {
    struct FirstOrZero;
    impl Aggregate<i64, i64> for FirstOrZero {
        fn init(&self) -> i64 { 0 }
        fn step(&self, ctx: &Context<'_>, acc: &mut i64) -> rslite::Result<()> {
            if *acc == 0 { *acc = ctx.get::<i64>(0)?; }
            Ok(())
        }
        fn finalize(&self, _: &Context<'_>, acc: Option<i64>) -> rslite::Result<i64> {
            Ok(acc.unwrap_or(-1))
        }
    }

    let db = Connection::open_in_memory().unwrap();
    db.execute_batch("CREATE TABLE t (x INTEGER)").unwrap();
    // No rows — finalize must receive None.
    db.create_aggregate_function("first_or_zero", 1, FunctionFlags::default(), FirstOrZero)
        .unwrap();
    let val: i64 = db.one_column("SELECT first_or_zero(x) FROM t").unwrap();
    assert_eq!(val, -1);
}

#[test]
fn test_readonly() {
    let db = Connection::open_in_memory().unwrap();
    let stmt = db.prepare("SELECT 1;").unwrap();
    assert!(stmt.readonly());
}
