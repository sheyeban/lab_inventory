use rusqlite::Connection;
use crate::{db, models::User, ui};

pub fn menu(conn: &Connection, user: &User) {
    loop {
        println!("\n--- Categories ---");
        println!("1) list (search)");
        println!("2) create");
        println!("3) update");
        println!("4) delete");
        println!("0) back");

        match ui::input_u32("Выбор: ") {
            1 => {
                let s = ui::input("search: ");
                for c in db::list_categories(conn, &s).expect("ошибка") {
                    println!("[{}] {}", c.id, c.name);
                }
            }
            2 => {
                let name = ui::input("name: ");
                match db::create_category(conn, &name) {
                    Ok(_) => { let _ = db::audit(conn, Some(user.id), "CATEGORY_CREATE", &format!("name={}", name)); println!("✅ создано"); }
                    Err(e) => println!("❌ {}", e),
                }
            }
            3 => {
                let id = ui::input_i64("id: ");
                let name = ui::input("new name: ");
                match db::update_category(conn, id, &name) {
                    Ok(_) => { let _ = db::audit(conn, Some(user.id), "CATEGORY_UPDATE", &format!("id={}", id)); println!("✅ обновлено"); }
                    Err(e) => println!("❌ {}", e),
                }
            }
            4 => {
                let id = ui::input_i64("id: ");
                match db::delete_category(conn, id) {
                    Ok(_) => { let _ = db::audit(conn, Some(user.id), "CATEGORY_DELETE", &format!("id={}", id)); println!("✅ удалено"); }
                    Err(e) => println!("❌ {}", e),
                }
            }
            0 => break,
            _ => println!("Неверно."),
        }
    }
}
