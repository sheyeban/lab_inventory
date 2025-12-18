use rusqlite::Connection;

use crate::{db, models::{Role, User}, ui};

pub fn menu(conn: &Connection, user: &User) {
    loop {
        println!("\n--- Inventory ---");
        println!("1) list (search/filter)");
        println!("2) add");

        match user.role {
            Role::LabManager => {
                println!("3) edit main fields (name/inv_num/category/vendor)");
                println!("4) change status");
                println!("5) delete");
            }
            Role::Assistant => {
                println!("3) change status");
            }
        }

        println!("0) back");

        match ui::input_u32("Выбор: ") {
            1 => list(conn),
            2 => add(conn, user),
            3 => {
                if user.role == Role::LabManager { edit_main(conn, user) }
                else { change_status(conn, user) }
            }
            4 => {
                if user.role == Role::LabManager { change_status(conn, user) }
                else { println!("Недоступно."); }
            }
            5 => {
                if user.role == Role::LabManager { delete(conn, user) }
                else { println!("Недоступно."); }
            }
            0 => break,
            _ => println!("Неверно."),
        }
    }
}

fn list(conn: &Connection) {
    let search = ui::input("search (name/inv_num, empty=all): ");
    let status_filter = ui::choose_status_optional();
    let items = db::list_items(conn, &search, status_filter).expect("ошибка list_items");

    if items.is_empty() { println!("Пусто."); return; }
    for it in items {
        println!(
            "[{}] name='{}' inv_num='{}' status={:?} category_id={} vendor_id={}",
            it.id, it.name, it.inv_num, it.status, it.category_id, it.vendor_id
        );
    }
}

fn add(conn: &Connection, user: &User) {
    let name = ui::input("name: ");
    let inv_num = ui::input("inv_num: ");
    let status = ui::choose_status_required();

    println!("Подсказка: сначала создай Category/Vendor (меню Categories/Vendors), потом сюда.");
    let category_id = ui::input_i64("category_id: ");
    let vendor_id = ui::input_i64("vendor_id: ");

    match db::create_item(conn, &name, &inv_num, &status, category_id, vendor_id) {
        Ok(_) => { let _ = db::audit(conn, Some(user.id), "ITEM_CREATE", &format!("inv_num={}", inv_num)); println!("✅ добавлено"); }
        Err(e) => println!("❌ {}", e),
    }
}

fn edit_main(conn: &Connection, user: &User) {
    let inv_num_old = ui::input("inv_num (что меняем): ");
    let name = ui::input("new name: ");
    let inv_num_new = ui::input("new inv_num: ");
    let category_id = ui::input_i64("new category_id: ");
    let vendor_id = ui::input_i64("new vendor_id: ");

    match db::update_item_main(conn, &inv_num_old, &name, &inv_num_new, category_id, vendor_id) {
        Ok(_) => { let _ = db::audit(conn, Some(user.id), "ITEM_UPDATE_MAIN", &format!("old={}, new={}", inv_num_old, inv_num_new)); println!("✅ обновлено"); }
        Err(e) => println!("❌ {}", e),
    }
}

fn change_status(conn: &Connection, user: &User) {
    let inv_num = ui::input("inv_num: ");
    let status = ui::choose_status_required();

    match db::update_item_status(conn, &inv_num, &status) {
        Ok(_) => { let _ = db::audit(conn, Some(user.id), "ITEM_STATUS_CHANGE", &format!("inv_num={}", inv_num)); println!("✅ статус обновлён"); }
        Err(e) => println!("❌ {}", e),
    }
}

fn delete(conn: &Connection, user: &User) {
    let inv_num = ui::input("inv_num: ");
    match db::delete_item(conn, &inv_num) {
        Ok(_) => { let _ = db::audit(conn, Some(user.id), "ITEM_DELETE", &format!("inv_num={}", inv_num)); println!("✅ удалено"); }
        Err(e) => println!("❌ {}", e),
    }
}
