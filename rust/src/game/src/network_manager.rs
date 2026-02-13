use crate::message_type::MessageType;
use godot::classes::{INode, Node};
use godot::global::godot_print;
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};
use snl::GameSocket;

const SERVER_IP: &str = "127.0.0.1:3630";

#[derive(GodotClass)]
#[class(base=Node)]
struct NetworkManager {
    socket: Option<GameSocket>,

    base: Base<Node>
}

#[godot_api]
impl INode for NetworkManager {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Network Manager initialized!");
        Self { socket: None, base }
    }

    fn process(&mut self, _delta: f64) {
        let buffer = &mut [];
        if let Some(socket) = self.socket.as_mut() {
            socket.poll(buffer);
        }
    }

    fn ready(&mut self) {
        let socket = GameSocket::new("127.0.0.1:3631");

        match socket {
            Ok(socket) => {
                self.socket = Some(socket);
                let mut message: Vec<u8> = vec![];
                self.send_message(MessageType::Helo, &mut message);
            }
            Err(e) => godot_print!("Error connecting to server: {}", e),
        }
    }
}

impl NetworkManager {
    fn send_message(&self, message_type: MessageType, buffer: &mut Vec<u8>) {
        godot_print!("Send message");
        let mut message_content: Vec<u8> = vec![];
        message_content.insert(0, message_type as u8);
        message_content.append(buffer);

        if let Some(socket) = self.socket.as_ref() {
            match socket.send(SERVER_IP, message_content.as_slice()) {
                Ok(size) => {
                    godot_print!("Message Sent of size {}", size);
                }
                Err(e) => godot_print!("Error sending message: {}", e),
            }
        }
    }
}
