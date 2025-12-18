pub mod inventory;
pub mod maintenance;
pub mod categories;
pub mod vendors;
pub mod users;
pub mod audit;

use rusqlite::Connection;
use crate::models::{Role, User};

pub fn run_role_menu(conn: &Connection, user: &User) {
    match user.role {
        Role::LabManager => lab_manager_menu(conn, user),
        Role::Assistant => assistant_menu(conn, user),
    }
}

fn lab_manager_menu(conn: &Connection, user: &User) {
    loop {
        println!("\n=== LabManager MENU ===");
        println!("1) Inventory");
        println!("2) Maintenance");
        println!("3) Categories");
        println!("4) Vendors");
        println!("5) Users");
        println!("6) Audit log");
        println!("0) Logout");

        match crate::ui::input_u32("Выбор: ") {
            1 => inventory::menu(conn, user),
            2 => maintenance::menu(conn, user),
            3 => categories::menu(conn, user),
            4 => vendors::menu(conn, user),
            5 => users::menu(conn, user),
            6 => audit::menu(conn),
            0 => { let _ = crate::db::audit(conn, Some(user.id), "USER_LOGOUT", &format!("username={}", user.username)); break; }
            _ => println!("Неверно."),
        }
    }
}

fn assistant_menu(conn: &Connection, user: &User) {
    loop {
        println!("\n=== Assistant MENU ===");
        println!("1) Inventory");
        println!("2) Maintenance");
        println!("0) Logout");

        match crate::ui::input_u32("Выбор: ") {
            1 => inventory::menu(conn, user),
            2 => maintenance::menu(conn, user),
            0 => { let _ = crate::db::audit(conn, Some(user.id), "USER_LOGOUT", &format!("username={}", user.username)); break; }
            _ => println!("Неверно."),
        }
    }
}
