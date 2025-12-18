use rusqlite::Connection;
use crate::{db, models::User, ui};

pub fn menu(conn: &Connection, user: &User) {
    loop {
        println!("\n--- Vendors ---");
        println!("1) list (search)");
        println!("2) create");
        println!("3) update");
        println!("4) delete");
        println!("0) back");

        match ui::input_u32("Выбор: ") {
            1 => {
                let s = ui::input("search: ");
                for v in db::list_vendors(conn, &s).expect("ошибка") {
                    println!("[{}] {}", v.id, v.name);
                }
            }
            2 => {
                let name = ui::input("name: ");
                match db::create_vendor(conn, &name) {
                    Ok(_) => { let _ = db::audit(conn, Some(user.id), "VENDOR_CREATE", &format!("name={}", name)); println!("✅ создано"); }
                    Err(e) => println!("❌ {}", e),
                }
            }
            3 => {
                let id = ui::input_i64("id: ");
                let name = ui::input("new name: ");
                match db::update_vendor(conn, id, &name) {
                    Ok(_) => { let _ = db::audit(conn, Some(user.id), "VENDOR_UPDATE", &format!("id={}", id)); println!("✅ обновлено"); }
                    Err(e) => println!("❌ {}", e),
                }
            }
            4 => {
                let id = ui::input_i64("id: ");
                match db::delete_vendor(conn, id) {
                    Ok(_) => { let _ = db::audit(conn, Some(user.id), "VENDOR_DELETE", &format!("id={}", id)); println!("✅ удалено"); }
                    Err(e) => println!("❌ {}", e),
                }
            }
            0 => break,
            _ => println!("Неверно."),
        }
    }
}
