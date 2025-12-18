use rusqlite::Connection;
use bcrypt::{hash, DEFAULT_COST};

use crate::{db, models::{Role, User}, ui};

pub fn menu(conn: &Connection, user: &User) {
    loop {
        println!("\n--- Users ---");
        println!("1) list (search)");
        println!("2) create assistant");
        println!("3) update role");
        println!("4) reset password");
        println!("5) delete user");
        println!("0) back");

        match ui::input_u32("Выбор: ") {
            1 => {
                let s = ui::input("search: ");
                for u in db::list_users(conn, &s).expect("ошибка") {
                    println!("[{}] {} {:?}", u.id, u.username, u.role);
                }
            }
            2 => {
                let username = ui::input("username: ");
                let password = ui::input("password: ");
                let password_hash = hash(password, DEFAULT_COST).expect("hash err");
                match db::create_user(conn, &username, &password_hash, &Role::Assistant) {
                    Ok(_) => { let _ = db::audit(conn, Some(user.id), "USER_CREATE", &format!("username={}", username)); println!("✅ создан assistant"); }
                    Err(e) => println!("❌ {}", e),
                }
            }
            3 => {
                let id = ui::input_i64("user_id: ");
                println!("role: 1) lab_manager 2) assistant");
                let role = match ui::input_u32("Выбор: ") {
                    1 => Role::LabManager,
                    _ => Role::Assistant,
                };
                match db::update_user_role(conn, id, &role) {
                    Ok(_) => { let _ = db::audit(conn, Some(user.id), "USER_ROLE_UPDATE", &format!("user_id={}", id)); println!("✅ роль обновлена"); }
                    Err(e) => println!("❌ {}", e),
                }
            }
            4 => {
                let id = ui::input_i64("user_id: ");
                let new_pass = ui::input("new password: ");
                let password_hash = hash(new_pass, DEFAULT_COST).expect("hash err");
                match db::update_user_password(conn, id, &password_hash) {
                    Ok(_) => { let _ = db::audit(conn, Some(user.id), "USER_PASSWORD_RESET", &format!("user_id={}", id)); println!("✅ пароль обновлён"); }
                    Err(e) => println!("❌ {}", e),
                }
            }
            5 => {
                let id = ui::input_i64("user_id: ");
                match db::delete_user(conn, id) {
                    Ok(_) => { let _ = db::audit(conn, Some(user.id), "USER_DELETE", &format!("user_id={}", id)); println!("✅ удалено"); }
                    Err(e) => println!("❌ {}", e),
                }
            }
            0 => break,
            _ => println!("Неверно."),
        }
    }
}
