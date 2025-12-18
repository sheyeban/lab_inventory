use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension, Result};

use crate::models::{AuditLog, Category, InventoryItem, ItemStatus, Role, User, Vendor};

fn now_ts() -> String {
    Utc::now().to_rfc3339()
}

pub fn open_db() -> Result<Connection> {
    let conn = Connection::open("db.sqlite")?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    Ok(conn)
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS vendors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS inventory_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            inv_num TEXT NOT NULL UNIQUE,
            status TEXT NOT NULL,
            category_id INTEGER NOT NULL,
            vendor_id INTEGER NOT NULL,
            FOREIGN KEY(category_id) REFERENCES categories(id),
            FOREIGN KEY(vendor_id) REFERENCES vendors(id)
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

        CREATE INDEX IF NOT EXISTS idx_items_name ON inventory_items(name);
        CREATE INDEX IF NOT EXISTS idx_items_inv ON inventory_items(inv_num);
        CREATE INDEX IF NOT EXISTS idx_audit_action ON audit_log(action);
        "#,
    )?;
    Ok(())
}

// ---------- AUDIT ----------
pub fn audit(conn: &Connection, user_id: Option<i64>, action: &str, details: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO audit_log (timestamp, user_id, action, details) VALUES (?1, ?2, ?3, ?4)",
        params![now_ts(), user_id, action, details],
    )?;
    Ok(())
}

pub fn list_audit(conn: &Connection, search: &str) -> Result<Vec<AuditLog>> {
    let mut out = Vec::new();
    if search.trim().is_empty() {
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, user_id, action, details
             FROM audit_log ORDER BY id DESC LIMIT 200",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(AuditLog {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                user_id: row.get(2)?,
                action: row.get(3)?,
                details: row.get(4)?,
            })
        })?;
        for r in rows { out.push(r?); }
    } else {
        let like = format!("%{}%", search);
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, user_id, action, details
             FROM audit_log
             WHERE action LIKE ?1 OR details LIKE ?1
             ORDER BY id DESC LIMIT 200",
        )?;
        let rows = stmt.query_map(params![like], |row| {
            Ok(AuditLog {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                user_id: row.get(2)?,
                action: row.get(3)?,
                details: row.get(4)?,
            })
        })?;
        for r in rows { out.push(r?); }
    }
    Ok(out)
}

// ---------- USERS CRUD ----------
pub fn users_count(conn: &Connection) -> Result<i64> {
    conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))
}

pub fn create_user(conn: &Connection, username: &str, password_hash: &str, role: &Role) -> Result<()> {
    conn.execute(
        "INSERT INTO users (username, password_hash, role) VALUES (?1, ?2, ?3)",
        params![username, password_hash, role.to_db()],
    )?;
    Ok(())
}

pub fn get_user_by_username(conn: &Connection, username: &str) -> Result<Option<(i64, String, Role)>> {
    conn.query_row(
        "SELECT id, password_hash, role FROM users WHERE username = ?1",
        params![username],
        |row| {
            let id: i64 = row.get(0)?;
            let hash: String = row.get(1)?;
            let role_str: String = row.get(2)?;
            Ok((id, hash, Role::from_db(&role_str)))
        },
    ).optional()
}

pub fn list_users(conn: &Connection, search: &str) -> Result<Vec<User>> {
    let mut out = Vec::new();
    if search.trim().is_empty() {
        let mut stmt = conn.prepare("SELECT id, username, role FROM users ORDER BY id")?;
        let rows = stmt.query_map([], |row| {
            let role_str: String = row.get(2)?;
            Ok(User { id: row.get(0)?, username: row.get(1)?, role: Role::from_db(&role_str) })
        })?;
        for r in rows { out.push(r?); }
    } else {
        let like = format!("%{}%", search);
        let mut stmt = conn.prepare("SELECT id, username, role FROM users WHERE username LIKE ?1 ORDER BY id")?;
        let rows = stmt.query_map(params![like], |row| {
            let role_str: String = row.get(2)?;
            Ok(User { id: row.get(0)?, username: row.get(1)?, role: Role::from_db(&role_str) })
        })?;
        for r in rows { out.push(r?); }
    }
    Ok(out)
}

pub fn update_user_role(conn: &Connection, user_id: i64, role: &Role) -> Result<()> {
    conn.execute("UPDATE users SET role = ?1 WHERE id = ?2", params![role.to_db(), user_id])?;
    Ok(())
}

pub fn update_user_password(conn: &Connection, user_id: i64, password_hash: &str) -> Result<()> {
    conn.execute("UPDATE users SET password_hash = ?1 WHERE id = ?2", params![password_hash, user_id])?;
    Ok(())
}

pub fn delete_user(conn: &Connection, user_id: i64) -> Result<()> {
    conn.execute("DELETE FROM users WHERE id = ?1", params![user_id])?;
    Ok(())
}

// ---------- CATEGORY CRUD ----------
pub fn create_category(conn: &Connection, name: &str) -> Result<()> {
    conn.execute("INSERT INTO categories (name) VALUES (?1)", params![name])?;
    Ok(())
}

pub fn list_categories(conn: &Connection, search: &str) -> Result<Vec<Category>> {
    let mut out = Vec::new();
    if search.trim().is_empty() {
        let mut stmt = conn.prepare("SELECT id, name FROM categories ORDER BY name")?;
        let rows = stmt.query_map([], |row| Ok(Category { id: row.get(0)?, name: row.get(1)? }))?;
        for r in rows { out.push(r?); }
    } else {
        let like = format!("%{}%", search);
        let mut stmt = conn.prepare("SELECT id, name FROM categories WHERE name LIKE ?1 ORDER BY name")?;
        let rows = stmt.query_map(params![like], |row| Ok(Category { id: row.get(0)?, name: row.get(1)? }))?;
        for r in rows { out.push(r?); }
    }
    Ok(out)
}

pub fn update_category(conn: &Connection, id: i64, name: &str) -> Result<()> {
    conn.execute("UPDATE categories SET name = ?1 WHERE id = ?2", params![name, id])?;
    Ok(())
}

pub fn delete_category(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM categories WHERE id = ?1", params![id])?;
    Ok(())
}

// ---------- VENDOR CRUD ----------
pub fn create_vendor(conn: &Connection, name: &str) -> Result<()> {
    conn.execute("INSERT INTO vendors (name) VALUES (?1)", params![name])?;
    Ok(())
}

pub fn list_vendors(conn: &Connection, search: &str) -> Result<Vec<Vendor>> {
    let mut out = Vec::new();
    if search.trim().is_empty() {
        let mut stmt = conn.prepare("SELECT id, name FROM vendors ORDER BY name")?;
        let rows = stmt.query_map([], |row| Ok(Vendor { id: row.get(0)?, name: row.get(1)? }))?;
        for r in rows { out.push(r?); }
    } else {
        let like = format!("%{}%", search);
        let mut stmt = conn.prepare("SELECT id, name FROM vendors WHERE name LIKE ?1 ORDER BY name")?;
        let rows = stmt.query_map(params![like], |row| Ok(Vendor { id: row.get(0)?, name: row.get(1)? }))?;
        for r in rows { out.push(r?); }
    }
    Ok(out)
}

pub fn update_vendor(conn: &Connection, id: i64, name: &str) -> Result<()> {
    conn.execute("UPDATE vendors SET name = ?1 WHERE id = ?2", params![name, id])?;
    Ok(())
}

pub fn delete_vendor(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM vendors WHERE id = ?1", params![id])?;
    Ok(())
}

// ---------- INVENTORY CRUD + search/filter ----------
pub fn create_item(
    conn: &Connection,
    name: &str,
    inv_num: &str,
    status: &ItemStatus,
    category_id: i64,
    vendor_id: i64,
) -> Result<()> {
    conn.execute(
        "INSERT INTO inventory_items (name, inv_num, status, category_id, vendor_id)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![name, inv_num, status.to_db(), category_id, vendor_id],
    )?;
    Ok(())
}

pub fn list_items(conn: &Connection, search: &str, status_filter: Option<ItemStatus>) -> Result<Vec<InventoryItem>> {
    let mut out = Vec::new();

    match (search.trim().is_empty(), status_filter) {
        (true, None) => {
            let mut stmt = conn.prepare(
                "SELECT id, name, inv_num, status, category_id, vendor_id
                 FROM inventory_items ORDER BY id DESC",
            )?;
            let rows = stmt.query_map([], |row| {
                let s: String = row.get(3)?;
                Ok(InventoryItem {
                    id: row.get(0)?, name: row.get(1)?, inv_num: row.get(2)?,
                    status: ItemStatus::from_db(&s),
                    category_id: row.get(4)?, vendor_id: row.get(5)?,
                })
            })?;
            for r in rows { out.push(r?); }
        }
        (false, None) => {
            let like = format!("%{}%", search);
            let mut stmt = conn.prepare(
                "SELECT id, name, inv_num, status, category_id, vendor_id
                 FROM inventory_items
                 WHERE name LIKE ?1 OR inv_num LIKE ?1
                 ORDER BY id DESC",
            )?;
            let rows = stmt.query_map(params![like], |row| {
                let s: String = row.get(3)?;
                Ok(InventoryItem {
                    id: row.get(0)?, name: row.get(1)?, inv_num: row.get(2)?,
                    status: ItemStatus::from_db(&s),
                    category_id: row.get(4)?, vendor_id: row.get(5)?,
                })
            })?;
            for r in rows { out.push(r?); }
        }
        (true, Some(st)) => {
            let mut stmt = conn.prepare(
                "SELECT id, name, inv_num, status, category_id, vendor_id
                 FROM inventory_items WHERE status = ?1
                 ORDER BY id DESC",
            )?;
            let rows = stmt.query_map(params![st.to_db()], |row| {
                let s: String = row.get(3)?;
                Ok(InventoryItem {
                    id: row.get(0)?, name: row.get(1)?, inv_num: row.get(2)?,
                    status: ItemStatus::from_db(&s),
                    category_id: row.get(4)?, vendor_id: row.get(5)?,
                })
            })?;
            for r in rows { out.push(r?); }
        }
        (false, Some(st)) => {
            let like = format!("%{}%", search);
            let mut stmt = conn.prepare(
                "SELECT id, name, inv_num, status, category_id, vendor_id
                 FROM inventory_items
                 WHERE (name LIKE ?1 OR inv_num LIKE ?1) AND status = ?2
                 ORDER BY id DESC",
            )?;
            let rows = stmt.query_map(params![like, st.to_db()], |row| {
                let s: String = row.get(3)?;
                Ok(InventoryItem {
                    id: row.get(0)?, name: row.get(1)?, inv_num: row.get(2)?,
                    status: ItemStatus::from_db(&s),
                    category_id: row.get(4)?, vendor_id: row.get(5)?,
                })
            })?;
            for r in rows { out.push(r?); }
        }
    }

    Ok(out)
}

pub fn get_item_by_inv_num(conn: &Connection, inv_num: &str) -> Result<Option<InventoryItem>> {
    conn.query_row(
        "SELECT id, name, inv_num, status, category_id, vendor_id
         FROM inventory_items WHERE inv_num = ?1",
        params![inv_num],
        |row| {
            let s: String = row.get(3)?;
            Ok(InventoryItem {
                id: row.get(0)?, name: row.get(1)?, inv_num: row.get(2)?,
                status: ItemStatus::from_db(&s),
                category_id: row.get(4)?, vendor_id: row.get(5)?,
            })
        },
    ).optional()
}

pub fn update_item_main(
    conn: &Connection,
    inv_num_old: &str,
    name: &str,
    inv_num_new: &str,
    category_id: i64,
    vendor_id: i64,
) -> Result<()> {
    conn.execute(
        "UPDATE inventory_items
         SET name = ?1, inv_num = ?2, category_id = ?3, vendor_id = ?4
         WHERE inv_num = ?5",
        params![name, inv_num_new, category_id, vendor_id, inv_num_old],
    )?;
    Ok(())
}

pub fn update_item_status(conn: &Connection, inv_num: &str, status: &ItemStatus) -> Result<()> {
    conn.execute("UPDATE inventory_items SET status = ?1 WHERE inv_num = ?2", params![status.to_db(), inv_num])?;
    Ok(())
}

pub fn delete_item(conn: &Connection, inv_num: &str) -> Result<()> {
    conn.execute("DELETE FROM inventory_items WHERE inv_num = ?1", params![inv_num])?;
    Ok(())
}

// ---------- MAINTENANCE ----------
pub fn add_maintenance(conn: &Connection, item_id: i64, description: &str, by_user_id: i64) -> Result<()> {
    conn.execute(
        "INSERT INTO maintenance_log (item_id, timestamp, description, by_user_id)
         VALUES (?1, ?2, ?3, ?4)",
        params![item_id, now_ts(), description, by_user_id],
    )?;
    Ok(())
}

pub fn list_maintenance(conn: &Connection, item_id: i64) -> Result<Vec<crate::models::MaintenanceLog>> {
    let mut out = Vec::new();
    let mut stmt = conn.prepare(
        "SELECT id, item_id, timestamp, description, by_user_id
         FROM maintenance_log WHERE item_id = ?1 ORDER BY id DESC",
    )?;
    let rows = stmt.query_map(params![item_id], |row| {
        Ok(crate::models::MaintenanceLog {
            id: row.get(0)?,
            item_id: row.get(1)?,
            timestamp: row.get(2)?,
            description: row.get(3)?,
            by_user_id: row.get(4)?,
        })
    })?;
    for r in rows { out.push(r?); }
    Ok(out)
}

pub fn delete_maintenance(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM maintenance_log WHERE id = ?1", params![id])?;
    Ok(())
}
