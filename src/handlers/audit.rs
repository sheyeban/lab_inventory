use rusqlite::Connection;
use crate::{db, ui};

pub fn menu(conn: &Connection) {
    let s = ui::input("search (action/details, empty=all): ");
    let logs = db::list_audit(conn, &s).expect("ошибка list_audit");
    for l in logs {
        println!("[{}] {} user_id={:?} {} {}", l.id, l.timestamp, l.user_id, l.action, l.details);
    }
    ui::pause();
}
