use crate::message_type::{MessageHeader, MessageType};
use godot::classes::{INode, Node};
use godot::global::godot_print;
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};
use snl::GameSocket;

const SERVER_IP: &str = "127.0.0.1:3630";

#[derive(Debug)]
pub enum ConnectionState {
    NotConnected,
    Connecting,
    Connected,
    Spurious,
}

#[derive(GodotClass)]
#[class(base=Node)]
struct NetworkManager {
    socket: Option<GameSocket>,
    connection_state: ConnectionState,
    connection_timeout: f64,

    base: Base<Node>,
}

#[godot_api]

impl INode for NetworkManager {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Network Manager initialized!");
        Self {
            socket: None,
            base,
            connection_state: ConnectionState::NotConnected,
            connection_timeout: 0.0,
        }
    }

    fn process(&mut self, delta: f64) {
        let mut buf = [0; 1500];
        if let Some(socket) = self.socket.as_mut() {
            match socket.poll(&mut buf) {
                Some((size, _)) => {
                    let buf = &mut buf[..size];

                    let message_header = MessageHeader::from_data(buf[0]);
                    match message_header.get_message_type() {
                        MessageType::Helo => self.set_connection_state(ConnectionState::Connecting),
                        MessageType::Hsk => self.set_connection_state(ConnectionState::Connected),
                        MessageType::Ping => self.set_connection_state(ConnectionState::Connected),
                        MessageType::Data => self.handle_data(message_header, &buf[1..]),
                    }
                }
                None => {}
            }
        }
        self.connection_timeout += delta;
        if self.connection_timeout > 0.100 {
            self.handle_timeout()
        }
    }

    fn ready(&mut self) {
        let socket = GameSocket::new("127.0.0.1:0");

        match socket {
            Ok(socket) => {
                self.socket = Some(socket);
                let mut message: Vec<u8> = vec![];
                self.send_message(MessageType::Helo, &mut message);
            }
            Err(e) => godot_print!(
                "Error conne

cting to server: {}",
                e
            ),
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
                Ok(_) => {}
                Err(e) => godot_print!("Error sending message: {}", e),
            }
        }
    }

    fn set_connection_state(&mut self, connection_state: ConnectionState) {
        self.connection_state = connection_state;
        self.connection_timeout = 0.0;
    }

    fn handle_timeout(&mut self) {
        match self.connection_state {
            ConnectionState::NotConnected => {
                let mut message: Vec<u8> = vec![];
                self.send_message(MessageType::Helo, &mut message);
            }
            ConnectionState::Connecting => {
                let mut message: Vec<u8> = vec![];
                self.send_message(MessageType::Hsk, &mut message);
            }
            ConnectionState::Connected => {}
            ConnectionState::Spurious => {}
        }

        self.connection_timeout = 0.0;
    }

    fn handle_data(&self, message_header: MessageHeader, _buffer: &[u8]) {
        println!("Receive Data");
        if message_header.is_rpc() {
            println!("Receive RPC");
        } else {
            println!("Receive Replication Data");
        }
    }
}
