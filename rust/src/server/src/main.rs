mod message_type;
mod network_manager;

use crate::network_manager::NetworkManager;

const SERVER_IP: &str = "127.0.0.1:3630";

fn main() {
    let socket = NetworkManager::new(SERVER_IP);

    println!("Server started");

    match socket {
        Ok(socket) => loop {
            match socket.poll() {
                Ok(_) => {}
                Err(e) => println!("Error: {}", e),
            }
        },
        Err(_) => println!("CPT"),
    }
}
