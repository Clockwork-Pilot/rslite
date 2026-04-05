use pub_interface::{Connection, Value};

fn main() -> pub_interface::Result<()> {
    // Open an in-memory database (statically linked C API).
    let mut db = Connection::open("test.db")?;

    // Create a table
    db.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER, name TEXT, age INTEGER)")?;

    // Insert data using parameterized queries (safe from SQL injection)
    db.execute_with_params(
        "INSERT INTO users VALUES (?, ?, ?)",
        &[Value::Integer(1), Value::Text("Alice".to_string()), Value::Integer(30)],
    )?;

    db.execute_with_params(
        "INSERT INTO users VALUES (?, ?, ?)",
        &[Value::Integer(2), Value::Text("Bob".to_string()), Value::Integer(25)],
    )?;

    // Query with parameters (SQL injection safe)
    let rows = db.query_with_params(
        "SELECT id, name, age FROM users WHERE age > ?",
        &[Value::Integer(20)],
    )?;

    println!("Users older than 20:");
    for row in rows.iter() {
        println!("  {:?}", row);
    }

    Ok(())
}
