use crate::message_header::{DataType, MessageHeader, MessageType};
use godot::builtin::Vector2;
use godot::classes::{INode, Label, Node, PackedScene};
use godot::global::godot_print;
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass, Node2D};
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
struct GDNetworkManager {
    socket: Option<GameSocket>,
    connection_state: ConnectionState,
    connection_timeout: f64,
    ping_sent: u32,

    client_id: u32,
    replicated_nodes: HashMap<u32, Gd<Node2D>>,

    base: Base<Node>,
    #[export]
    player_scene: Option<Gd<PackedScene>>,

    player: Option<Gd<Node>>,
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
            client_id: 0,
            player_scene: None,
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

                    godot_print!(
                        "Received message : {:?} | {:?}",
                        message_header.get_message_type(),
                        message_header.get_data_type()
                    );

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

    fn exit_tree(&mut self) {
        self.disconnect_socket(true);
    }
}

impl GDNetworkManager {
    fn send_message(&self, message_type: MessageType, buffer: &mut Vec<u8>) {
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
                if (self.ping_sent < 3) {
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
        godot_print!("Receive Data");
        self.connection_timeout = 0.0;

        match message_header.get_data_type() {
            DataType::None => {}
            DataType::Rpc => {}
            DataType::Replication => {
                let net_id =
                    u32::from_le_bytes(buffer[0..4].try_into().expect("Slice de mauvaise taille"));
                //let class_id =
                //    u32::from_le_bytes(buffer[4..8].try_into().expect("Slice de mauvaise taille"));
                //let x =
                //    f32::from_le_bytes(buffer[8..12].try_into().expect("Slice de mauvaise taille"));
                //let y = f32::from_le_bytes(
                //    buffer[12..16].try_into().expect("Slice de mauvaise taille"),
                //);

                if let Some(replicated_node) = self.replicated_nodes.get(&net_id) {
                    //replicated_node.set_position(Vector2::new(x, y));
                } else {
                    self.spawn_player_scene(buffer);
                }
            }
            DataType::Spawn => {
                self.spawn_player_scene(buffer);
            }
        }
    }

    fn spawn_player_scene(&mut self, buffer: &[u8]) {
        let Some(scene) = &self.player_scene else {
            godot_print!("Enemy scene not assigned!");
            return;
        };

        let player_node = scene
            .clone()
            .instantiate()
            .expect("Failed to instantiate scene");

        let net_id = u32::from_le_bytes(buffer[0..4].try_into().expect("Slice de mauvaise taille"));
        let x = f32::from_le_bytes(buffer[8..12].try_into().expect("Slice de mauvaise taille"));
        let y = f32::from_le_bytes(buffer[12..16].try_into().expect("Slice de mauvaise taille"));

        self.base()
            .get_tree()
            .get_root()
            .unwrap()
            .add_child(&player_node);

        if let Ok(mut node_2d) = player_node.clone().try_cast::<Node2D>() {
            let position = Vector2::new(x, y);
            godot_print!("Position of scene: {}", position);
            node_2d.set_position(position);
            self.replicated_nodes.insert(net_id, node_2d);
        }

        self.player = Some(player_node);
    }
}
