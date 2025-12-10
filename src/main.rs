use std::{io, net::IpAddr, process};

fn get_user_input() {
    let mut user_input = String::new();
    println!("Введите IP:");
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            check_ip_address(user_input.trim());
        }
        Err(e) => {
            eprintln!("Ошибка ввода: {}", e);
            process::exit(1);
        }
    }
}

fn main() {
    get_user_input();
}

fn check_ip_address(user_input: &str) {
    match user_input.parse::<IpAddr>() {
        Ok(ip) => {
            println!("✅ Это IP-адрес: {}", ip);
        }
        Err(_) => {
            eprintln!("❌ Это не IP-адрес");
        }
    }
}