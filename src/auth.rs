use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::Connection;

use crate::{db, models::{Role, User}};

pub fn seed_pushnyavka_if_empty(conn: &Connection) {
    let count = db::users_count(conn).expect("ошибка подсчета юзеров");
    if count == 0 {
        let password_hash = hash("pushnyavka", DEFAULT_COST).expect("ошибка хеширования");
        db::create_user(conn, "pushnyavka", &password_hash, &Role::LabManager)
            .expect("ошибка создания пушнявки(админ)!");
        let _ = db::audit(conn, None, "USER_SEED", "created initial lab_manager pushnyavka");
    }
}

pub fn login(conn: &Connection, username: &str, password: &str) -> Option<User> {
    let found = db::get_user_by_username(conn, username).ok().flatten()?;
    let (id, password_hash, role) = found;

    if verify(password, &password_hash).unwrap_or(false) {
        let _ = db::audit(conn, Some(id), "USER_LOGIN", &format!("username={}", username));
        Some(User { id, username: username.to_string(), role })
    } else {
        let _ = db::audit(conn, Some(id), "USER_LOGIN_FAIL", &format!("username={}", username));
        None
    }
}
