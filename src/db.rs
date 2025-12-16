use rusqlite::{Connection, Result};

pub fn open_db() -> Result<Connection> {
    Connection::open("db.sqlite")
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
    r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS inventory_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            inv_num TEXT NOT NULL UNIQUE,
            status TEXT NOT NULL,
            vendor TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS maintenance_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            item_id INTEGER NOT NULL,
            timestamp TEXT NOT NULL,
            description TEXT NOT NULL,
            by_user_id INTEGER NOT NULL,
            FOREIGN KEY(item_id) REFERENCES inventory_items(id),
            FOREIGN KEY(by_user_id) REFERENCES users(id)
        );

        CREATE TABLE IF NOT EXISTS audit_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            user_id INTEGER,
            action TEXT NOT NULL,
            details TEXT NOT NULL
        );
        "#,
    )?;
    Ok(())
}

pub fn users_count(conn: &Connection) -> Result<i64> {
    conn.query_row(     //возвращает 1 значение
        "SELECT COUNT(*) FROM users",
        [],
        |row| row.get(0),
    )
}

pub fn create_user(
    conn: &Connection,
    username: &str,
    password_hash: &str,
    role: &str
) -> Result<()> {
    conn.execute(
        "INSERT INTO users (username, password_hash, role) VALUES (?1, ?2, ?3)",
        (username, password_hash, role)
    )?;
    Ok(())
}