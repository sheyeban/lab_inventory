use rusqlite::Connection;

use crate::{db, models::User, ui};

pub fn menu(conn: &Connection, user: &User) {
    loop {
        println!("\n--- Maintenance ---");
        println!("1) add");
        println!("2) list by inv_num");
        println!("3) delete by id (только если знаешь id)");
        println!("0) back");

        match ui::input_u32("Выбор: ") {
            1 => add(conn, user),
            2 => list(conn),
            3 => delete(conn, user),
            0 => break,
            _ => println!("Неверно."),
        }
    }
}

fn add(conn: &Connection, user: &User) {
    let inv_num = ui::input("inv_num: ");
    let Some(item) = db::get_item_by_inv_num(conn, &inv_num).expect("ошибка поиска item") else {
        println!("Не найдено."); return;
    };

    let description = ui::input("description: ");
    match db::add_maintenance(conn, item.id, &description, user.id) {
        Ok(_) => { let _ = db::audit(conn, Some(user.id), "MAINT_ADD", &format!("inv_num={}", inv_num)); println!("✅ добавлено"); }
        Err(e) => println!("❌ {}", e),
    }
}

fn list(conn: &Connection) {
    let inv_num = ui::input("inv_num: ");
    let Some(item) = db::get_item_by_inv_num(conn, &inv_num).expect("ошибка поиска item") else {
        println!("Не найдено."); return;
    };

    let logs = db::list_maintenance(conn, item.id).expect("ошибка list_maintenance");
    if logs.is_empty() { println!("Пусто."); return; }
    for l in logs {
        println!("[{}] {} by_user_id={} {}", l.id, l.timestamp, l.by_user_id, l.description);
    }
}

fn delete(conn: &Connection, user: &User) {
    let id = ui::input_i64("maintenance id: ");
    match db::delete_maintenance(conn, id) {
        Ok(_) => { let _ = db::audit(conn, Some(user.id), "MAINT_DELETE", &format!("maintenance_id={}", id)); println!("✅ удалено"); }
        Err(e) => println!("❌ {}", e),
    }
}
