use crate::linking_context::GDLinkingContext;
use common::message_header::{DataType, MessageHeader, MessageType};
use common::stream_reader::StreamReader;
use common::stream_writer::StreamWriter;
use godot::classes::{INode, Node};
use godot::global::godot_print;
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass};
use snl::GameSocket;

const SERVER_IP: &str = "127.0.0.1:3630";

#[derive(Debug, Clone)]
pub enum ConnectionState {
    NotConnected,
    Connecting,
    Connected,
    Spurious,
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GDNetworkManager {
    socket: Option<GameSocket>,
    connection_state: ConnectionState,
    connection_timeout: f64,
    ping_sent: u32,

    pub client_id: u32,
    base: Base<Node>,
}

#[godot_api]
impl INode for GDNetworkManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            socket: None,
            connection_state: ConnectionState::NotConnected,
            connection_timeout: 0.0,
            ping_sent: 0,
            client_id: 0,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let mut buf = [0; 1500];
        if let Some(socket) = self.socket.as_mut() {
            match socket.poll(&mut buf) {
                Some((size, _)) => {
                    let buf = &mut buf[..size];

                    let mut stream_reader = StreamReader::new(buf.to_vec());
                    let mut message_header = MessageHeader::new();
                    stream_reader.read_serializable(&mut message_header);

                    match message_header.message_type {
                        MessageType::Helo => self.set_connection_state(ConnectionState::Connecting),
                        MessageType::Hsk => {
                            self.set_connection_state(ConnectionState::Connected);
                            self.client_id = stream_reader.read_u32();
                            godot_print!("ClientID : {:?}", self.client_id);
                            let mut buffer: Vec<u8> = vec![];
                            self.send_message(MessageType::Data, &mut buffer);
                        }
                        MessageType::Ping => self.set_connection_state(ConnectionState::Connected),
                        MessageType::Data => {
                            self.handle_data(message_header, stream_reader.get_rest_buffer())
                        }
                        MessageType::Bye => self.disconnect_socket(false),
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

    fn exit_tree(&mut self) {
        self.disconnect_socket(true);
    }

    fn ready(&mut self) {
        let socket = GameSocket::new("127.0.0.1:0");

        match socket {
            Ok(socket) => {
                self.socket = Some(socket);
                self.handle_timeout();
            }
            Err(e) => godot_print!("Error connecting to server: {}", e),
        }
    }
}

impl GDNetworkManager {
    pub fn send_message(&self, message_type: MessageType, buffer: &mut Vec<u8>) {
        let mut stream_writer = StreamWriter::new(vec![]);
        stream_writer.write_serializable(MessageHeader::init(message_type, DataType::Rpc));
        stream_writer.write_bytes(buffer);

        if let Some(socket) = self.socket.as_ref() {
            match socket.send(SERVER_IP, stream_writer.get_data()) {
                Ok(_) => {}
                Err(e) => godot_print!("Error sending message: {}", e),
            }
        }
    }

    fn set_connection_state(&mut self, connection_state: ConnectionState) {
        self.connection_state = connection_state;
        self.connection_timeout = 0.0;
        godot_print!("Connection state: {:?}", self.connection_state);
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
            ConnectionState::Connected => {
                //self.set_connection_state(ConnectionState::Spurious)
            }
            ConnectionState::Spurious => {
                if self.ping_sent < 3 {
                    let mut message: Vec<u8> = vec![];
                    self.send_message(MessageType::Ping, &mut message);
                    self.ping_sent += 1;
                } else {
                    self.disconnect_socket(false)
                }
            }
        }

        self.connection_timeout = 0.0;
    }

    pub fn disconnect_socket(&mut self, send_bye: bool) {
        self.ping_sent = 0;
        if send_bye {
            let mut stream_writer = StreamWriter::new(vec![]);
            stream_writer.write_u32(self.client_id);
            self.send_message(MessageType::Bye, &mut stream_writer.get_data().to_vec());
        }
        self.set_connection_state(ConnectionState::NotConnected);
    }

    fn get_linking_context(&mut self) -> Gd<GDLinkingContext> {
        self.base()
            .get_node_as::<GDLinkingContext>("%GDLinkingContext")
    }

    fn handle_data(&mut self, message_header: MessageHeader, buffer: &[u8]) {
        self.connection_timeout = 0.0;
        match message_header.data_type {
            DataType::None => {}
            DataType::Rpc => {}
            DataType::Replication => {
                self.get_linking_context()
                    .bind_mut()
                    .handle_snapshot(buffer.to_vec());
            }
            DataType::Despawn => {
                self.despawn_replicated_node(buffer);
            }
        }
    }

    fn despawn_replicated_node(&mut self, buffer: &[u8]) {
        let mut stream_reader = StreamReader::new(buffer.to_vec());
        let net_id = stream_reader.read_u32();

        self.get_linking_context().bind_mut().despawn(net_id);
    }
}
