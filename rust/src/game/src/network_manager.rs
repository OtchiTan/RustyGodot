use crate::linking_context::LinkingContext;
use common::message_header::{DataType, MessageHeader, MessageType};
use common::serializer::Serializer;
use godot::classes::notify::NodeNotification;
use godot::classes::{INode, Label, Node};
use godot::global::godot_print;
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass};
use snl::GameSocket;
use std::collections::HashMap;

const SERVER_IP: &str = "127.0.0.1:3630";

#[derive(Debug, Clone)]
pub enum ConnectionState {
    NotConnected,
    Connecting,
    Connected,
    Spurious,
}

impl ConnectionState {
    fn to_string(self) -> String {
        match self {
            ConnectionState::NotConnected => "NotConnected".into(),
            ConnectionState::Connecting => "Connecting".into(),
            ConnectionState::Connected => "Connected".into(),
            ConnectionState::Spurious => "Spurious".into(),
        }
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GDNetworkManager {
    socket: Option<GameSocket>,
    connection_state: ConnectionState,
    connection_timeout: f64,
    ping_sent: u32,

    linking_context: LinkingContext,
    client_id: u32,
    replicated_nodes: HashMap<Gd<Node>, u32>,
    replicated_nodes_id: HashMap<u32, Gd<Node>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for GDNetworkManager {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Network Manager initialized!");

        Self {
            socket: None,
            base,
            connection_state: ConnectionState::NotConnected,
            connection_timeout: 0.0,
            ping_sent: 0,
            linking_context: LinkingContext::new(),
            client_id: 0,
            replicated_nodes: HashMap::new(),
            replicated_nodes_id: HashMap::new(),
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
                        MessageType::Hsk => {
                            self.set_connection_state(ConnectionState::Connected);
                            if let Some(slice) = buf.get(1..5) {
                                let bytes: [u8; 4] = slice.try_into().unwrap();
                                self.client_id = u32::from_le_bytes(bytes);
                                godot_print!("ClientID : {:?}", self.client_id);

                                let mut buffer: Vec<u8> = vec![];
                                self.send_message(MessageType::Data, &mut buffer);
                            }
                        }
                        MessageType::Ping => self.set_connection_state(ConnectionState::Connected),
                        MessageType::Data => self.handle_data(message_header, &buf[1..]),
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

#[godot_api]
impl GDNetworkManager {
    pub fn send_message(&self, message_type: MessageType, buffer: &mut Vec<u8>) {
        let mut message_content: Vec<u8> =
            vec![MessageHeader::new(message_type, DataType::Rpc).get_data()];
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

        let mut state_connection_label = self.base().get_node_as::<Label>("%ConnectionStateLabel");
        let connection_state_text = self.connection_state.clone().to_string();
        state_connection_label.set_text(connection_state_text.as_str());
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

    fn disconnect_socket(&mut self, send_bye: bool) {
        self.ping_sent = 0;
        if send_bye {
            let mut serializer = Serializer::new(vec![]);
            let _ = &mut serializer << self.client_id;
            self.send_message(MessageType::Bye, &mut serializer.get_data().to_vec());
        }
        self.set_connection_state(ConnectionState::NotConnected);
    }

    fn handle_data(&mut self, message_header: MessageHeader, buffer: &[u8]) {
        self.connection_timeout = 0.0;

        match message_header.get_data_type() {
            DataType::None => {}
            DataType::Rpc => {}
            DataType::Replication => {
                let mut serializer = Serializer::new(buffer.to_vec());
                let mut net_id: u32 = 0;
                let _ = &mut serializer >> &mut net_id;

                if let Some(replicated_node) = self.get_node_by_id(net_id) {
                    self.linking_context
                        .deserialize(replicated_node.clone(), buffer[8..].to_vec());
                } else {
                    self.spawn_replicated_node(buffer);
                }
            }
        }
    }

    pub fn get_node_by_id(&self, id: u32) -> Option<Gd<Node>> {
        self.replicated_nodes_id.get(&id).cloned()
    }

    fn spawn_replicated_node(&mut self, buffer: &[u8]) {
        let mut serializer = Serializer::new(buffer.to_vec());
        let mut net_id: u32 = 0;
        let mut type_id: u32 = 0;
        let _ = &mut serializer >> &mut net_id;
        let _ = &mut serializer >> &mut type_id;

        let replicated_node = self.linking_context.spawn(type_id as usize);
        self.linking_context
            .deserialize(replicated_node.clone(), buffer[8..].to_vec());

        self.base_mut().add_child(&replicated_node);

        self.replicated_nodes_id
            .insert(net_id, replicated_node.clone());
        self.replicated_nodes
            .insert(replicated_node.clone(), net_id);
    }

    #[func]
    pub fn replicate_node(&mut self, node: Gd<Node>) {
        let mut data = self.linking_context.serialize(node.clone());
        if let Some(net_id) = self.replicated_nodes.get(&node) {
            let mut serializer = Serializer::new(vec![]);
            let _ = &mut serializer << *net_id;
            let mut buffer = serializer.get_data().to_vec();
            buffer.append(&mut data);
            self.send_message(MessageType::Data, &mut buffer);
        }
    }
}
