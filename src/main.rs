use std::net::IpAddr;
fn main() {
    let input = "192.168.0.1";
    match input.parse::<IpAddr>() {
        Ok(ip) => {
            println!("Это IP-адрес: {}", ip);
        }
        Err(_) => {
            eprintln!("Это не IP-адрес")
        }
    }
}
