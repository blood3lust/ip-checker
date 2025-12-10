use std::{io, net::IpAddr, process};
#[derive(Debug, PartialEq)]
pub enum InputType {
    Ip(IpAddr),
    Invalid,
}

pub fn classify_input(input: &str) -> InputType {
    match input.parse::<IpAddr>() {
        Ok(ip) => InputType::Ip(ip),
        Err(_) => InputType::Invalid,
    }
}

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
    match classify_input(user_input) {
        InputType::Ip(ip) => println!("✅ Это IP-адрес: {}", ip),
        InputType::Invalid => eprintln!("❌ Это не IP-адрес"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ipv4() {
        assert_eq!(classify_input("192.168.1.1"), InputType::Ip("192.168.1.1".parse().unwrap()));
    }

    #[test]
    fn test_valid_ipv6() {
        assert_eq!(classify_input("::1"), InputType::Ip("::1".parse().unwrap()));
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(classify_input("google.com"), InputType::Invalid);
        assert_eq!(classify_input("256.1.1.1"), InputType::Invalid);
        assert_eq!(classify_input(""), InputType::Invalid);
    }
}