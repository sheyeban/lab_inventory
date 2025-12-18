use std::io::{self, Write};

use crate::models::ItemStatus;

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn input_u32(prompt: &str) -> u32 {
    loop {
        let s = input(prompt);
        if let Ok(n) = s.parse::<u32>() {
            return n;
        }
        println!("Введите число.");
    }
}

pub fn input_i64(prompt: &str) -> i64 {
    loop {
        let s = input(prompt);
        if let Ok(n) = s.parse::<i64>() {
            return n;
        }
        println!("Введите число.");
    }
}

pub fn pause() {
    let _ = input("Нажмите Enter...");
}

pub fn choose_status_required() -> ItemStatus {
    loop {
        println!("Статус:");
        println!("1) available (в наличии)");
        println!("2) in_service (на обслуживании)");
        println!("3) written_off (списано)");

        match input_u32("Выбор: ") {
            1 => return ItemStatus::Available,
            2 => return ItemStatus::InService,
            3 => return ItemStatus::WrittenOff,
            _ => println!("Неверный выбор."),
        }
    }
}

pub fn choose_status_optional() -> Option<ItemStatus> {
    println!("Фильтр по статусу:");
    println!("0) без фильтра");
    println!("1) available");
    println!("2) in_service");
    println!("3) written_off");

    match input_u32("Выбор: ") {
        1 => Some(ItemStatus::Available),
        2 => Some(ItemStatus::InService),
        3 => Some(ItemStatus::WrittenOff),
        _ => None,
    }
}
