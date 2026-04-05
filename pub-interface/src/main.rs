use pub_interface::{Connection, Value};

fn main() -> pub_interface::Result<()> {
    // Open an in-memory database (statically linked C API).
    let mut db = Connection::open(":memory:")?;

    // Create a table
    db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER)")?;

    println!("=== Basic Transaction ===");

    // Insert data using explicit transaction
    db.begin()?;
    println!("Transaction started (in_transaction={})", db.in_transaction());

    db.execute_with_params(
        "INSERT INTO users VALUES (?, ?, ?)",
        &[Value::Integer(1), Value::Text("Alice".to_string()), Value::Integer(30)],
    )?;

    db.execute_with_params(
        "INSERT INTO users VALUES (?, ?, ?)",
        &[Value::Integer(2), Value::Text("Bob".to_string()), Value::Integer(25)],
    )?;

    db.commit()?;
    println!("Transaction committed (in_transaction={})", db.in_transaction());

    // Closure-based transaction with auto-rollback on error
    println!("\n=== Closure-based Transaction ===");
    db.transaction(|conn| {
        println!("  Inside transaction");
        conn.execute_with_params(
            "INSERT INTO users VALUES (?, ?, ?)",
            &[Value::Integer(3), Value::Text("Charlie".to_string()), Value::Integer(35)],
        )?;
        println!("  Charlie inserted");
        Ok(())
    })?;
    println!("  Transaction auto-committed");

    // Transaction with rollback demo
    println!("\n=== Rollback Demo ===");
    let result: pub_interface::Result<()> = db.transaction(|conn| {
        println!("  Attempting insert in transaction");
        conn.execute_with_params(
            "INSERT INTO users VALUES (?, ?, ?)",
            &[Value::Integer(4), Value::Text("Dave".to_string()), Value::Integer(40)],
        )?;
        println!("  Dave inserted, but will rollback");
        Err(pub_interface::Error::Database("simulated error".to_string()))
    });
    match result {
        Ok(()) => println!("  Transaction succeeded"),
        Err(_) => println!("  Transaction rolled back (simulated)"),
    }

    // Query all users
    println!("\n=== All Users ===");
    let rows = db.query("SELECT id, name, age FROM users ORDER BY id")?;
    for row in rows.iter() {
        println!("  {:?}", row);
    }

    Ok(())
}
