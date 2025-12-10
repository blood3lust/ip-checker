use std::{io, net::IpAddr, process};
#[derive(Debug, PartialEq)]
pub enum InputType {
    Ip(IpAddr),
    Domain(String),
    Invalid,
}

pub fn classify_input(input: &str) -> InputType {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return InputType::Invalid;
    }

    // Сначала пробуем IP
    if let Ok(ip) = trimmed.parse::<IpAddr>() {
        return InputType::Ip(ip);
    }

    // Потом — домен
    if is_valid_domain(trimmed) {
        return InputType::Domain(trimmed.to_string());
    }

    InputType::Invalid
}
fn is_valid_domain(s: &str) -> bool {
    if s.is_empty() || !s.contains('.') {
        return false;
    }
    if s.starts_with('.') || s.ends_with('.') || s.starts_with('-') || s.ends_with('-') {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.iter().any(|part| part.is_empty() || part.len() > 63) {
        return false;
    }
    // Новое правило: хотя бы одна часть должна содержать букву
    if !parts
        .iter()
        .any(|part| part.chars().any(|c| c.is_ascii_alphabetic()))
    {
        return false; // только цифры → не домен
    }
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
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
        InputType::Domain(domain) => println!("🌐 Это домен: {}", domain),
        InputType::Invalid => eprintln!("❌ Неизвестный формат"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ipv4() {
        assert_eq!(
            classify_input("192.168.1.1"),
            InputType::Ip("192.168.1.1".parse().unwrap())
        );
    }

    #[test]
    fn test_valid_ipv6() {
        assert_eq!(classify_input("::1"), InputType::Ip("::1".parse().unwrap()));
    }

    #[test]
    fn test_invalid_input() {
        // Теперь "google.com" — Domain, а не Invalid
        assert_eq!(classify_input("256.1.1.1"), InputType::Invalid);
        assert_eq!(classify_input(""), InputType::Invalid);
        assert_eq!(classify_input("example@com"), InputType::Invalid);
    }

    #[test]
    fn test_valid_domain() {
        assert_eq!(
            classify_input("example.com"),
            InputType::Domain("example.com".to_string())
        );
        assert_eq!(
            classify_input("rust-lang.org"),
            InputType::Domain("rust-lang.org".to_string())
        );
    }

    #[test]
    fn test_invalid_domain() {
        // "192.168.1" — на самом деле Domain по твоей логике!
        // Но можно запретить домены, состоящие только из цифр и точек.
        assert_eq!(classify_input(".example.com"), InputType::Invalid);
        assert_eq!(classify_input("example..com"), InputType::Invalid);
        assert_eq!(classify_input("-example.com"), InputType::Invalid);
        assert_eq!(classify_input("example.com-"), InputType::Invalid);
    }
}
