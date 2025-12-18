mod models;
mod db;
mod auth;
mod ui;
mod handlers;

fn main() {
    let conn = db::open_db().expect("бд не открылась");
    db::init_db(&conn).expect("таблицы не создались");

    auth::seed_pushnyavka_if_empty(&conn);

    loop {
        println!("\n=== LOGIN ===");
        println!("seed lab_manager: pushnyavka / pushnyavka");
        println!("1) Login");
        println!("0) Exit");

        match ui::input_u32("Выбор: ") {
            1 => {
                let username = ui::input("Логин: ");
                let password = ui::input("Пароль: ");

                if let Some(user) = auth::login(&conn, &username, &password) {
                    handlers::run_role_menu(&conn, &user);
                } else {
                    println!("Неверный логин или пароль.");
                }
            }
            0 => break,
            _ => println!("Неверно."),
        }
    }

    println!("Пока 👋");
}
