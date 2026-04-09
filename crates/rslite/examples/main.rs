use rslite::{Connection, params};

fn main() -> rslite::Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute_batch(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER NOT NULL);"
    )?;

    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        params!["Alice", 30i64],
    )?;
    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        params!["Bob", 25i64],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, age FROM users ORDER BY id")?;
    let mut rows = stmt.query(())?;
    while let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        let name: String = row.get(1)?;
        let age: i64 = row.get(2)?;
        println!("{}: {} (age {})", id, name, age);
    }

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM users",
        (),
        |row| row.get(0),
    )?;
    println!("Total users: {}", count);

    Ok(())
}
