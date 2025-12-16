mod models;
mod db;

use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let conn = db::open_db().expect("бд не открылась");
    db::init_db(&conn).expect("таблицы не создались");

    let count = db::users_count(&conn).expect("ошибка подсчета юзеров");

    if count == 0 {
        let password_hash = hash("pushnyavka", DEFAULT_COST).expect("ошибка хеширования");

        db::create_user(
            &conn,
            "pushnyavka",
            &password_hash,
            "lab_manager",
        ).expect("ошибка создания пушнявки(админ)!");

        println!("пушнявка создана");
    } else {
        println!("юзеры уже существуют");
    }
}
