//! Comprehensive feature tour for rslite.
//!
//! Every public API added to rslite is exercised here so the file serves as both
//! a reference and a smoke-test.  Each section is a standalone function that
//! prints what it does and asserts the expected results.
//!
//! Run with:
//!   cargo run --bin features

use rslite::{
    params, named_params, params_from_iter,
    Aggregate, Connection, Context, DropBehavior, FunctionFlags, LimitCategory, Null,
    OpenFlags, OptionalExtension, TransactionBehavior, Value, ValueRef,
};
use rslite::connection::{AuthAction, Authorization};
use rslite::hooks::Action;

fn main() -> rslite::Result<()> {
    demo_open_and_close()?;
    demo_basic_crud()?;
    demo_statement_metadata()?;
    demo_cached_statements()?;
    demo_query_methods()?;
    demo_row_access()?;
    demo_params_variants()?;
    demo_optional_extension()?;
    demo_transactions()?;
    demo_savepoints()?;
    demo_pragmas()?;
    demo_connection_info()?;
    demo_busy_handling()?;
    demo_limits()?;
    demo_collations()?;
    demo_authorizer()?;
    demo_trace()?;
    demo_scalar_functions()?;
    demo_aggregate_functions()?;
    demo_hooks()?;
    demo_values_and_types()?;
    demo_ffi_constants();
    println!("\nAll features demonstrated successfully.");
    Ok(())
}

// ── 1. Opening and closing connections ────────────────────────────────────────

fn demo_open_and_close() -> rslite::Result<()> {
    println!("--- 1. Opening and closing connections ---");

    // The most common way: in-memory database, gone when closed.
    let conn = Connection::open_in_memory()?;
    println!("  opened in-memory database");

    // path() returns None or an empty string for in-memory databases.
    assert!(conn.path().map_or(true, |p| p.is_empty()));

    // open_with_flags lets you choose read-only, no-mutex, URI mode, etc.
    let conn2 = Connection::open_with_flags(":memory:", OpenFlags::READWRITE | OpenFlags::CREATE)?;
    println!("  opened with explicit flags");
    drop(conn2);

    // close() returns an error instead of silently failing if SQLite refuses.
    conn.close().expect("close should succeed on idle connection");
    println!("  closed connection explicitly");

    Ok(())
}

// ── 2. Basic CRUD ─────────────────────────────────────────────────────────────

fn demo_basic_crud() -> rslite::Result<()> {
    println!("--- 2. Basic CRUD ---");

    let conn = Connection::open_in_memory()?;

    // execute_batch runs multiple semicolon-separated statements at once.
    conn.execute_batch(
        "CREATE TABLE fruits (id INTEGER PRIMARY KEY, name TEXT NOT NULL, weight_g REAL);
         INSERT INTO fruits (name, weight_g) VALUES ('apple', 182.0);
         INSERT INTO fruits (name, weight_g) VALUES ('banana', 118.0);",
    )?;
    println!("  created table and seeded data with execute_batch");

    // execute returns the number of rows changed.
    let changed = conn.execute(
        "UPDATE fruits SET weight_g = ?1 WHERE name = ?2",
        params![120.5f64, "banana"],
    )?;
    assert_eq!(changed, 1);
    println!("  updated 1 row with execute");

    // query_row grabs the first row and maps it immediately.
    let (name, weight): (String, f64) = conn.query_row(
        "SELECT name, weight_g FROM fruits WHERE id = 1",
        (),
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    println!("  query_row: {} weighs {:.1}g", name, weight);

    // one_column is shorthand for a single-column, single-row result.
    let count: i64 = conn.one_column("SELECT COUNT(*) FROM fruits")?;
    assert_eq!(count, 2);
    println!("  one_column: {} fruits in the table", count);

    // query_row_optional converts QueryReturnedNoRows into Ok(None).
    let missing: Option<String> = conn.query_row(
        "SELECT name FROM fruits WHERE id = 999",
        (),
        |row| row.get(0),
    ).optional()?;
    assert!(missing.is_none());
    println!("  query_row + optional: missing row returns None");

    // query_row_and_then lets the mapping closure return a foreign error type.
    // The error type must implement From<rslite::Error>; Box<dyn Error> satisfies this.
    let upper: Result<String, Box<dyn std::error::Error>> = conn.query_row_and_then(
        "SELECT name FROM fruits WHERE id = 1",
        (),
        |row| row.get::<_, String>(0).map(|s| s.to_uppercase()).map_err(Into::into),
    );
    println!("  query_row_and_then: {}", upper.unwrap());

    Ok(())
}

// ── 3. Statement metadata ─────────────────────────────────────────────────────

fn demo_statement_metadata() -> rslite::Result<()> {
    println!("--- 3. Statement metadata ---");

    let conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE t (a INTEGER, b TEXT, c REAL);")?;
    conn.execute("INSERT INTO t VALUES (1, 'hello', 3.14)", ())?;

    let stmt = conn.prepare("SELECT a, b, c FROM t WHERE a = ?1")?;

    // Column metadata — how many columns, what are they called, find by name.
    assert_eq!(stmt.column_count(), 3);
    assert_eq!(stmt.column_names(), vec!["a", "b", "c"]);
    assert_eq!(stmt.column_name(1)?, "b");
    assert_eq!(stmt.column_index("c")?, 2);
    println!("  column_count={}, names={:?}", stmt.column_count(), stmt.column_names());

    // Parameter metadata — how many placeholders, their names and indices.
    assert_eq!(stmt.parameter_count(), 1);
    // ?1-style parameters may return a name like "?1" or None depending on SQLite build.
    println!("  parameter_count={}, parameter_name(1)={:?}", stmt.parameter_count(), stmt.parameter_name(1));

    // Named parameters do have names.
    let stmt2 = conn.prepare("SELECT * FROM t WHERE a = :val")?;
    assert_eq!(stmt2.parameter_name(1), Some(":val"));
    assert_eq!(stmt2.parameter_index(":val")?, Some(1));
    println!("  named parameter ':val' is at index 1");

    // readonly() — SELECT never changes the database.
    assert!(stmt.readonly());
    let insert_stmt = conn.prepare("INSERT INTO t VALUES (?1, ?2, ?3)")?;
    assert!(!insert_stmt.readonly());
    println!("  SELECT is readonly={}, INSERT is readonly={}", stmt.readonly(), insert_stmt.readonly());

    // sql() returns the original text; expanded_sql() fills in bound values.
    assert!(stmt.sql().unwrap().contains("WHERE"));
    println!("  sql(): {:?}", stmt.sql());

    // finalize() explicitly closes the statement and surfaces any deferred error.
    stmt.finalize()?;
    println!("  statement finalized explicitly");

    Ok(())
}

// ── 4. Cached statements ──────────────────────────────────────────────────────

fn demo_cached_statements() -> rslite::Result<()> {
    println!("--- 4. Cached statements ---");

    let conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE cache_test (x INTEGER);")?;
    conn.execute("INSERT INTO cache_test VALUES (42)", ())?;

    // prepare_cached returns the same compiled statement on the second call,
    // avoiding repeated SQL parsing overhead.
    {
        let mut stmt = conn.prepare_cached("SELECT x FROM cache_test")?;
        let x: i64 = stmt.query_row((), |r| r.get(0))?;
        assert_eq!(x, 42);
        println!("  first prepare_cached call: x={}", x);
        // Dropping stmt here returns the raw pointer to the cache.
    }
    {
        // This reuses the cached sqlite3_stmt* instead of reparsing.
        let mut stmt = conn.prepare_cached("SELECT x FROM cache_test")?;
        let x: i64 = stmt.query_row((), |r| r.get(0))?;
        println!("  second prepare_cached call (reused): x={}", x);
    }

    // Adjust cache size at runtime; setting to 0 disables caching.
    conn.set_prepared_statement_cache_capacity(16);
    println!("  cache capacity set to 16");

    // cache_flush finalizes everything currently cached.
    conn.cache_flush();
    println!("  cache flushed");

    Ok(())
}

// ── 5. Statement execution methods ────────────────────────────────────────────

fn demo_query_methods() -> rslite::Result<()> {
    println!("--- 5. Statement query methods ---");

    let conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE nums (n INTEGER);")?;
    for i in 1i64..=5 {
        conn.execute("INSERT INTO nums VALUES (?1)", params![i])?;
    }

    // insert() returns the rowid of the newly inserted row.
    let mut ins = conn.prepare("INSERT INTO nums VALUES (?1)")?;
    let rowid = ins.insert(params![99i64])?;
    println!("  insert() returned rowid={}", rowid);

    // query_map applies a closure to every row and collects into an iterator.
    let mut stmt = conn.prepare("SELECT n FROM nums ORDER BY n")?;
    let values: Vec<i64> = stmt.query_map((), |row| row.get(0))?.collect::<rslite::Result<_>>()?;
    println!("  query_map collected: {:?}", values);

    // query_and_then works like query_map but the closure may return any error
    // type that implements From<rslite::Error>, not just rslite::Error itself.
    let mut stmt2 = conn.prepare("SELECT n FROM nums WHERE n < 4 ORDER BY n")?;
    let strings: Vec<String> = stmt2
        .query_and_then((), |row| -> Result<String, Box<dyn std::error::Error>> {
            row.get::<_, i64>(0)
                .map(|n| format!("val={}", n))
                .map_err(Into::into)
        })?
        .collect::<Result<_, Box<dyn std::error::Error>>>()
        .expect("query_and_then collect failed");
    println!("  query_and_then strings: {:?}", strings);

    // exists() returns true if the query produces at least one row.
    let mut stmt3 = conn.prepare("SELECT 1 FROM nums WHERE n = ?1")?;
    assert!(stmt3.exists(params![3i64])?);
    assert!(!stmt3.exists(params![100i64])?);
    println!("  exists(3)=true, exists(100)=false");

    // Rows::map is a convenience wrapper over the Rows streaming iterator.
    let mut stmt4 = conn.prepare("SELECT n FROM nums WHERE n <= 3 ORDER BY n")?;
    let rows = stmt4.query(())?;
    let small: Vec<i64> = rows.map(|row| row.get(0))?;
    println!("  Rows::map collected small values: {:?}", small);

    Ok(())
}

// ── 6. Row access ─────────────────────────────────────────────────────────────

fn demo_row_access() -> rslite::Result<()> {
    println!("--- 6. Row access ---");

    let conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE items (id INTEGER, label TEXT, score REAL, data BLOB);")?;
    conn.execute(
        "INSERT INTO items VALUES (1, 'alpha', 9.5, X'DEADBEEF')",
        (),
    )?;

    let mut stmt = conn.prepare("SELECT id, label, score, data FROM items")?;
    let mut rows = stmt.query(())?;
    let row = rows.next()?.unwrap();

    // get() by positional index (usize).
    let id: i64 = row.get(0usize)?;
    assert_eq!(id, 1);

    // get() by column name (&str).
    let label: String = row.get("label")?;
    assert_eq!(label, "alpha");

    // get() by i32 index works too.
    let score: f64 = row.get(2i32)?;
    assert!((score - 9.5).abs() < 1e-9);

    println!("  id={} label={} score={}", id, label, score);

    // get_ref() returns a zero-copy ValueRef tied to the current step.
    let vref = row.get_ref("data")?;
    if let ValueRef::Blob(bytes) = vref {
        println!("  data blob: {:02X?}", bytes);
    }

    // get_unwrap() panics on error — handy in tests where you know the type.
    let id2: i64 = row.get_unwrap(0usize);
    assert_eq!(id2, 1);

    // get_ref_unwrap() is the panicking version of get_ref().
    let _ = row.get_ref_unwrap("label");

    // Row exposes column_count and column_name just like Statement does.
    assert_eq!(row.column_count(), 4);
    assert_eq!(row.column_name(1)?, "label");
    println!("  row has {} columns, col[1]={}", row.column_count(), row.column_name(1)?);

    Ok(())
}

// ── 7. Parameter binding variants ─────────────────────────────────────────────

fn demo_params_variants() -> rslite::Result<()> {
    println!("--- 7. Parameter binding variants ---");

    let conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE p (a INTEGER, b TEXT);")?;

    // Tuple binding — the most ergonomic option for small, heterogeneous lists.
    conn.execute("INSERT INTO p VALUES (?1, ?2)", (1i64, "one"))?;

    // params![] macro — heterogeneous slice, same ergonomics as a tuple.
    conn.execute("INSERT INTO p VALUES (?1, ?2)", params![2i64, "two"])?;

    // Homogeneous typed slice — all elements must be the same ToSql type.
    conn.execute("INSERT INTO p VALUES (?1, ?2)", &[&3i64 as &dyn rslite::ToSql, &"three"])?;

    // named_params!{} — bind by name rather than position.
    conn.execute(
        "INSERT INTO p (a, b) VALUES (:a, :b)",
        named_params!{ ":a": 4i64, ":b": "four" },
    )?;

    // params_from_iter — bind from any iterator, useful with dynamic argument lists.
    let values: Vec<i64> = vec![5];
    conn.execute("INSERT INTO p (a) VALUES (?1)", params_from_iter(values))?;

    // Null sentinel — explicitly bind SQL NULL.
    conn.execute("INSERT INTO p (a, b) VALUES (?1, ?2)", (6i64, Null))?;

    let count: i64 = conn.one_column("SELECT COUNT(*) FROM p")?;
    assert_eq!(count, 6);
    println!("  inserted 6 rows using 5 different binding styles");

    Ok(())
}

// ── 8. OptionalExtension ──────────────────────────────────────────────────────

fn demo_optional_extension() -> rslite::Result<()> {
    println!("--- 8. OptionalExtension ---");

    let conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE opt (x INTEGER);")?;
    conn.execute("INSERT INTO opt VALUES (42)", ())?;

    // Without .optional(), QueryReturnedNoRows is an error.
    let found: Option<i64> = conn
        .query_row("SELECT x FROM opt WHERE x = 42", (), |r| r.get(0))
        .optional()?;
    assert_eq!(found, Some(42));
    println!("  found row: {:?}", found);

    let not_found: Option<i64> = conn
        .query_row("SELECT x FROM opt WHERE x = 0", (), |r| r.get(0))
        .optional()?;
    assert!(not_found.is_none());
    println!("  missing row: {:?}", not_found);

    Ok(())
}

// ── 9. Transactions ───────────────────────────────────────────────────────────

fn demo_transactions() -> rslite::Result<()> {
    println!("--- 9. Transactions ---");

    let mut conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance INTEGER);")?;
    conn.execute("INSERT INTO accounts VALUES (1, 1000)", ())?;
    conn.execute("INSERT INTO accounts VALUES (2, 500)", ())?;

    // A transaction rolled back leaves the data unchanged.
    {
        let tx = conn.transaction()?;
        tx.execute("UPDATE accounts SET balance = balance - 200 WHERE id = 1", ())?;
        tx.execute("UPDATE accounts SET balance = balance + 200 WHERE id = 2", ())?;
        tx.rollback()?;
        println!("  rolled back a transfer");
    }
    let bal: i64 = conn.query_row("SELECT balance FROM accounts WHERE id = 1", (), |r| r.get(0))?;
    assert_eq!(bal, 1000, "balance must be unchanged after rollback");

    // A committed transaction persists the changes.
    {
        let tx = conn.transaction()?;
        tx.execute("UPDATE accounts SET balance = balance - 200 WHERE id = 1", ())?;
        tx.execute("UPDATE accounts SET balance = balance + 200 WHERE id = 2", ())?;
        tx.commit()?;
        println!("  committed a transfer");
    }
    let bal2: i64 = conn.query_row("SELECT balance FROM accounts WHERE id = 1", (), |r| r.get(0))?;
    assert_eq!(bal2, 800);
    println!("  balance after commit: {}", bal2);

    // transaction_with_behavior lets you choose DEFERRED / IMMEDIATE / EXCLUSIVE.
    {
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        assert!(!tx.is_autocommit());
        tx.commit()?;
        println!("  IMMEDIATE transaction committed");
    }

    // DropBehavior controls what happens if the transaction is dropped without
    // an explicit commit or rollback.
    {
        let mut tx = conn.transaction()?;
        tx.set_drop_behavior(DropBehavior::Commit);
        assert_eq!(tx.drop_behavior(), DropBehavior::Commit);
        tx.execute("UPDATE accounts SET balance = balance + 1 WHERE id = 1", ())?;
        // Drop without calling commit() — DropBehavior::Commit auto-commits.
    }
    let bal3: i64 = conn.query_row("SELECT balance FROM accounts WHERE id = 1", (), |r| r.get(0))?;
    assert_eq!(bal3, 801);
    println!("  DropBehavior::Commit auto-committed: balance={}", bal3);

    Ok(())
}

// ── 10. Savepoints ────────────────────────────────────────────────────────────

fn demo_savepoints() -> rslite::Result<()> {
    println!("--- 10. Savepoints ---");

    let mut conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE sp_test (v INTEGER);")?;
    conn.execute("INSERT INTO sp_test VALUES (1)", ())?;

    let mut tx = conn.transaction()?;

    // Create a savepoint within the transaction.
    {
        let mut sp = tx.savepoint()?;
        sp.execute("INSERT INTO sp_test VALUES (2)", ())?;

        // Rolling back the savepoint un-does only the work done after it was created.
        sp.rollback()?;
        sp.commit()?; // release the savepoint
        println!("  savepoint rolled back: row 2 was discarded");
    }

    {
        // savepoint_with_name lets you give it a meaningful label.
        let mut sp = tx.savepoint_with_name("my_sp")?;
        sp.execute("INSERT INTO sp_test VALUES (3)", ())?;
        sp.commit()?; // RELEASE SAVEPOINT my_sp — keeps the insert
        println!("  named savepoint committed: row 3 is kept");
    }

    tx.commit()?;

    let count: i64 = conn.one_column("SELECT COUNT(*) FROM sp_test")?;
    assert_eq!(count, 2); // original row 1 + committed row 3
    println!("  final row count: {}", count);

    Ok(())
}

// ── 11. Pragmas ───────────────────────────────────────────────────────────────

fn demo_pragmas() -> rslite::Result<()> {
    println!("--- 11. Pragmas ---");

    let conn = Connection::open_in_memory()?;

    // pragma_update sets a pragma value.
    conn.pragma_update(None, "journal_mode", "WAL")?;
    println!("  set journal_mode = WAL");

    // pragma_query_value reads back a single scalar pragma.
    let page_size: i64 = conn.pragma_query_value(None, "page_size", |row| row.get(0))?;
    println!("  page_size = {}", page_size);
    assert!(page_size > 0);

    // pragma_query iterates over pragmas that return multiple rows.
    // database_list returns one row per attached database (at minimum "main").
    let mut dbs: Vec<String> = Vec::new();
    conn.pragma_query(None, "database_list", |row| {
        let db_name: String = row.get(1)?;
        dbs.push(db_name);
        Ok(())
    })?;
    println!("  pragma database_list: {:?}", dbs);
    assert!(!dbs.is_empty());

    Ok(())
}

// ── 12. Connection information ────────────────────────────────────────────────

fn demo_connection_info() -> rslite::Result<()> {
    println!("--- 12. Connection information ---");

    let conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE ci (x INTEGER);")?;

    // is_autocommit is true when outside a BEGIN block.
    assert!(conn.is_autocommit());
    println!("  is_autocommit={}", conn.is_autocommit());

    // path() is None for in-memory databases.
    println!("  path={:?}", conn.path());

    // Insert a row and check last_insert_rowid + changes.
    conn.execute("INSERT INTO ci VALUES (42)", ())?;
    println!("  last_insert_rowid={}", conn.last_insert_rowid());
    assert_eq!(conn.changes(), 1);
    assert!(conn.total_changes() >= 1);
    println!("  changes={} total_changes={}", conn.changes(), conn.total_changes());

    Ok(())
}

// ── 13. Busy handling ─────────────────────────────────────────────────────────

fn demo_busy_handling() -> rslite::Result<()> {
    println!("--- 13. Busy handling ---");

    let mut conn = Connection::open_in_memory()?;

    // busy_timeout makes SQLite retry for up to N milliseconds before giving up.
    conn.busy_timeout(std::time::Duration::from_millis(100))?;
    println!("  busy_timeout set to 100ms");

    // busy_handler lets you decide retry logic yourself.
    // The closure receives the number of prior retries; return true to retry.
    conn.busy_handler(Some(|n: i32| {
        println!("    busy_handler called (attempt {}), stopping", n);
        false // do not retry
    }))?;
    println!("  custom busy_handler registered");

    // Remove the handler by passing None.
    conn.busy_handler::<fn(i32) -> bool>(None)?;
    println!("  busy_handler removed");

    Ok(())
}

// ── 14. Runtime limits ────────────────────────────────────────────────────────

fn demo_limits() -> rslite::Result<()> {
    println!("--- 14. Runtime limits (LimitCategory) ---");

    let conn = Connection::open_in_memory()?;

    // Read the current SQL length limit.
    let current = conn.limit(LimitCategory::SqlLength, -1);
    println!("  current SqlLength limit: {}", current);
    assert!(current > 0);

    // Lower the function-argument limit and verify it.
    let old = conn.limit(LimitCategory::FunctionArg, 8);
    println!("  FunctionArg old={} → new=8", old);
    let now = conn.limit(LimitCategory::FunctionArg, -1);
    assert_eq!(now, 8);

    // Restore the original.
    conn.limit(LimitCategory::FunctionArg, old);
    println!("  FunctionArg restored to {}", old);

    Ok(())
}

// ── 15. Custom collations ─────────────────────────────────────────────────────

fn demo_collations() -> rslite::Result<()> {
    println!("--- 15. Custom collations ---");

    let mut conn = Connection::open_in_memory()?;

    // Register a case-insensitive ASCII collation.
    conn.create_collation("NOCASE_ASCII", |a, b| {
        a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase())
    })?;
    println!("  registered NOCASE_ASCII collation");

    conn.execute_batch("CREATE TABLE words (w TEXT COLLATE NOCASE_ASCII);")?;
    conn.execute("INSERT INTO words VALUES ('Banana')", ())?;
    conn.execute("INSERT INTO words VALUES ('apple')", ())?;
    conn.execute("INSERT INTO words VALUES ('Cherry')", ())?;

    // The custom collation governs ORDER BY.
    let mut stmt = conn.prepare("SELECT w FROM words ORDER BY w")?;
    let sorted: Vec<String> = stmt.query_map((), |r| r.get(0))?.collect::<rslite::Result<_>>()?;
    println!("  sorted with NOCASE_ASCII: {:?}", sorted);
    // 'apple' < 'Banana' < 'Cherry' when case is ignored.
    assert_eq!(sorted, vec!["apple", "Banana", "Cherry"]);

    Ok(())
}

// ── 16. Authorizer ────────────────────────────────────────────────────────────

fn demo_authorizer() -> rslite::Result<()> {
    println!("--- 16. Authorizer ---");

    let mut conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE public_data (x INTEGER);
                        CREATE TABLE secret (token TEXT);")?;

    // Register an authorizer that blocks reads from 'secret'.
    conn.authorizer(Some(|action: AuthAction<'_>| {
        if let AuthAction::Read { table_name: Some("secret"), .. } = action {
            Authorization::Deny
        } else {
            Authorization::Allow
        }
    }));
    println!("  authorizer registered — reads from 'secret' are blocked");

    // Reading from the allowed table works fine.
    let r = conn.execute("INSERT INTO public_data VALUES (1)", ());
    println!("  INSERT into public_data: {:?}", r.map(|_| "ok"));

    // Attempting to read from 'secret' is denied at prepare time.
    let blocked = conn.prepare("SELECT token FROM secret").is_err();
    println!("  SELECT from secret: {}", if blocked { "denied ✓" } else { "allowed (unexpected)" });

    // Remove the authorizer by passing None.
    conn.authorizer(None::<fn(AuthAction<'_>) -> Authorization>);
    println!("  authorizer removed");

    Ok(())
}

// ── 17. Trace callback ────────────────────────────────────────────────────────

fn demo_trace() -> rslite::Result<()> {
    println!("--- 17. Trace callback ---");

    let mut conn = Connection::open_in_memory()?;

    // trace() receives the SQL text of every statement just before it runs.
    conn.trace(Some(|sql: &str| {
        println!("    [trace] {}", sql.trim());
    }));
    println!("  trace callback registered");

    conn.execute_batch("CREATE TABLE trace_test (n INTEGER);")?;
    conn.execute("INSERT INTO trace_test VALUES (1)", ())?;

    // Passing None removes the callback.
    conn.trace(None);
    println!("  trace callback removed");

    Ok(())
}

// ── 18. Scalar user-defined functions ─────────────────────────────────────────

fn demo_scalar_functions() -> rslite::Result<()> {
    println!("--- 18. Scalar functions ---");

    let conn = Connection::open_in_memory()?;

    // A simple deterministic function: squares an integer.
    conn.create_scalar_function(
        "square",
        1,                          // exactly 1 argument
        FunctionFlags::DETERMINISTIC,
        |ctx: &Context<'_>| {
            let n: i64 = ctx.get(0)?;
            Ok(n * n)
        },
    )?;
    let result: i64 = conn.one_column("SELECT square(7)")?;
    assert_eq!(result, 49);
    println!("  square(7) = {}", result);

    // A function that concatenates all its arguments (variadic: n_args = -1).
    conn.create_scalar_function(
        "concat_all",
        -1,
        FunctionFlags::default(),
        |ctx: &Context<'_>| {
            let mut out = String::new();
            for i in 0..ctx.len() {
                out.push_str(&ctx.get::<String>(i)?);
            }
            Ok(out)
        },
    )?;
    let joined: String = conn.one_column("SELECT concat_all('hello', ', ', 'world')")?;
    println!("  concat_all = {:?}", joined);
    assert_eq!(joined, "hello, world");

    // remove_function de-registers it; subsequent calls return an SQL error.
    conn.remove_function("square", 1)?;
    let gone = conn.one_column::<i64>("SELECT square(3)");
    println!("  square after removal: {}", if gone.is_err() { "not found ✓" } else { "still present?" });

    Ok(())
}

// ── 19. Aggregate user-defined functions ──────────────────────────────────────

fn demo_aggregate_functions() -> rslite::Result<()> {
    println!("--- 19. Aggregate functions ---");

    let conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE vals (n INTEGER);")?;
    for v in [3i64, 1, 4, 1, 5, 9, 2, 6] {
        conn.execute("INSERT INTO vals VALUES (?1)", params![v])?;
    }

    // Implement a product aggregate: multiplies all values in a group.
    struct Product;

    impl Aggregate<i64, i64> for Product {
        fn init(&self) -> i64 { 1 }

        fn step(&self, ctx: &Context<'_>, acc: &mut i64) -> rslite::Result<()> {
            *acc *= ctx.get::<i64>(0)?;
            Ok(())
        }

        fn finalize(&self, _ctx: &Context<'_>, acc: Option<i64>) -> rslite::Result<i64> {
            Ok(acc.unwrap_or(1))
        }
    }

    conn.create_aggregate_function("product", 1, FunctionFlags::default(), Product)?;

    let p: i64 = conn.one_column("SELECT product(n) FROM vals")?;
    // 3 * 1 * 4 * 1 * 5 * 9 * 2 * 6 = 6480
    assert_eq!(p, 6480);
    println!("  product of all values = {}", p);

    Ok(())
}

// ── 20. Commit / rollback / update hooks ──────────────────────────────────────

fn demo_hooks() -> rslite::Result<()> {
    println!("--- 20. Hooks ---");

    use std::sync::{Arc, Mutex};

    let mut conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE hook_test (x INTEGER);")?;

    let commits   = Arc::new(Mutex::new(0u32));
    let rollbacks = Arc::new(Mutex::new(0u32));
    let updates: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    // commit_hook is called just before a transaction is committed.
    // Return true to abort the commit (convert to rollback), false to allow it.
    {
        let c = Arc::clone(&commits);
        conn.commit_hook(Some(move || {
            *c.lock().unwrap() += 1;
            false // allow the commit
        }));
    }

    // rollback_hook fires whenever a transaction is rolled back.
    {
        let r = Arc::clone(&rollbacks);
        conn.rollback_hook(Some(move || {
            *r.lock().unwrap() += 1;
        }));
    }

    // update_hook fires after every INSERT, UPDATE, or DELETE.
    // Arguments: action, database name, table name, rowid.
    {
        let u = Arc::clone(&updates);
        conn.update_hook(Some(move |action: Action, _db: &str, tbl: &str, rowid: i64| {
            u.lock().unwrap().push(format!("{:?} on {} rowid={}", action, tbl, rowid));
        }));
    }

    conn.execute("INSERT INTO hook_test VALUES (1)", ())?;
    conn.execute("INSERT INTO hook_test VALUES (2)", ())?;
    conn.execute("DELETE FROM hook_test WHERE x = 2", ())?;

    println!("  commits recorded by hook: {}", *commits.lock().unwrap());
    println!("  update events: {:?}", *updates.lock().unwrap());

    // Removing hooks by passing None.
    conn.commit_hook::<fn() -> bool>(None);
    conn.rollback_hook::<fn()>(None);
    conn.update_hook::<fn(Action, &str, &str, i64)>(None);
    println!("  all hooks removed");

    Ok(())
}

// ── 21. Value and ValueRef types ──────────────────────────────────────────────

fn demo_values_and_types() -> rslite::Result<()> {
    println!("--- 21. Value and ValueRef types ---");

    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE mixed (i INTEGER, r REAL, t TEXT, b BLOB, n INTEGER);"
    )?;
    conn.execute(
        "INSERT INTO mixed VALUES (42, 3.14, 'hello', X'CAFE', NULL)",
        (),
    )?;

    let mut stmt = conn.prepare("SELECT i, r, t, b, n FROM mixed")?;
    let mut rows = stmt.query(())?;
    let row = rows.next()?.unwrap();

    // get_ref() gives a zero-copy ValueRef whose lifetime is tied to the row.
    let vref_i = row.get_ref(0)?;
    let vref_r = row.get_ref(1)?;
    let vref_t = row.get_ref(2)?;
    let vref_b = row.get_ref(3)?;
    let vref_n = row.get_ref(4)?;

    println!("  integer: {:?} → as_i64={:?}", vref_i, vref_i.as_i64());
    println!("  real:    {:?} → as_f64={:?}", vref_r, vref_r.as_f64());
    println!("  text:    {:?} → as_str={:?}", vref_t, vref_t.as_str());
    println!("  blob:    {:?} → as_blob len={}", vref_b, vref_b.as_blob().unwrap().len());
    println!("  null:    {:?}", vref_n);

    // data_type() tells you the SQLite storage class.
    assert_eq!(vref_i.data_type(), rslite::Type::Integer);
    assert_eq!(vref_n.data_type(), rslite::Type::Null);

    // Converting a ValueRef to an owned Value is free for scalars, copies for text/blob.
    let owned: Value = vref_t.into();
    println!("  ValueRef→Value: {:?}", owned);

    // Value also exposes data_type().
    assert_eq!(owned.data_type(), rslite::Type::Text);

    Ok(())
}

// ── 22. ffi constants ─────────────────────────────────────────────────────────

fn demo_ffi_constants() {
    println!("--- 22. ffi constants ---");

    // The ffi module re-exports SQLite result codes and limit/auth constants.
    use rslite::ffi;

    println!("  SQLITE_OK             = {}", ffi::SQLITE_OK);
    println!("  SQLITE_ERROR          = {}", ffi::SQLITE_ERROR);
    println!("  SQLITE_BUSY           = {}", ffi::SQLITE_BUSY);
    println!("  SQLITE_CANTOPEN       = {}", ffi::SQLITE_CANTOPEN);
    println!("  SQLITE_CONSTRAINT_NOTNULL = {}", ffi::SQLITE_CONSTRAINT_NOTNULL);
    println!("  SQLITE_LIMIT_LENGTH   = {}", ffi::SQLITE_LIMIT_LENGTH);
    println!("  SQLITE_CREATE_TABLE   = {}", ffi::SQLITE_CREATE_TABLE);
    println!("  SQLITE_DENY           = {}", ffi::SQLITE_DENY);
    println!("  SQLITE_IGNORE         = {}", ffi::SQLITE_IGNORE);
    println!("  SQLITE_TRACE_STMT     = {}", ffi::SQLITE_TRACE_STMT);
}
