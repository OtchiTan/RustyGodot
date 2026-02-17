use common::message_header::{DataType, MessageHeader, MessageType};
use crate::player::GDPlayer;
use common::serializer::Serializer;
use godot::builtin::{Array, Vector2};
use godot::classes::{INode, Label, Node, Node2D, PackedScene};
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

    #[export]
    linking_context: Array<Gd<PackedScene>>,
    client_id: u32,
    replicated_nodes: HashMap<u32, Gd<Node>>,

    base: Base<Node>,

    player: Option<Gd<GDPlayer>>,
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
            linking_context: Array::new(),
            client_id: 0,
            player: None,
            replicated_nodes: HashMap::new(),
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
        let mut message_content: Vec<u8> = vec![];
        message_content.insert(
            0,
            MessageHeader::new(message_type, DataType::Rpc).get_data(),
        );
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
            ConnectionState::Connected => self.set_connection_state(ConnectionState::Spurious),
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
            let mut message: Vec<u8> = vec![];
            self.send_message(MessageType::Bye, &mut message);
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
                    if let Ok(mut _replicated_node) = replicated_node.try_cast::<Node2D>() {
                        let mut class_id: u32 = 0;
                        let mut x: f32 = 0.0;
                        let mut y: f32 = 0.0;
                        let _ = &mut serializer >> &mut class_id;
                        let _ = &mut serializer >> &mut x;
                        let _ = &mut serializer >> &mut y;

                        //replicated_node.set_position(Vector2::new(x, y));
                    }
                } else {
                    self.spawn_replicated_node(buffer);
                }
            }
        }
    }

    pub fn get_node_by_id(&self, id: u32) -> Option<Gd<Node>> {
        self.replicated_nodes.get(&id).cloned()
    }

    fn spawn_replicated_node(&mut self, buffer: &[u8]) {
        let mut serializer = Serializer::new(buffer.to_vec());
        let mut net_id: u32 = 0;
        let mut class_id: u32 = 0;
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let _ = &mut serializer >> &mut net_id;
        let _ = &mut serializer >> &mut class_id;
        let _ = &mut serializer >> &mut x;
        let _ = &mut serializer >> &mut y;

        let Some(scene) = &self.linking_context.get(class_id as usize) else {
            godot_print!("Scene not found");
            return;
        };

        let player_node = scene.instantiate_as::<Node>();

        self.base_mut().add_child(&player_node);

        if let Ok(player_node) = player_node.clone().try_cast::<Node2D>() {
            let position = Vector2::new(x, y);
            godot_print!("Position of scene: {}", position);
            player_node.clone().set_position(position);
            if let Ok(player_node) = player_node.get_child(0).unwrap().try_cast::<GDPlayer>() {
                player_node.clone().bind_mut().net_id = net_id;
                self.player = Some(player_node);
            }
        }

        self.replicated_nodes.insert(net_id, player_node.clone());
    }

    #[func]
    pub fn replicate_new_movement(&mut self, node: Gd<GDPlayer>, new_position: Vector2) {
        let mut serializer = Serializer::new(vec![]);
        let _ = &mut serializer << node.bind().net_id;
        let _ = &mut serializer << 0;
        let _ = &mut serializer << new_position.x;
        let _ = &mut serializer << new_position.y;
        self.send_message(MessageType::Data, &mut serializer.get_data().to_vec());
    }
}
