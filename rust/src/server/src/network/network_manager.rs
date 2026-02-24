use crate::network::connected_client::ConnectedClient;
use crate::replication::DestroyEntity;
use crate::replication::replicated_node::ReplicatedNode;
use crate::replication::replication_manager::{ClientEntityLink, ReplicationManager};
use bevy::prelude::{Commands, Resource};
use common::message_header::{DataType, MessageHeader, MessageType};
use common::serializer::Serializer;
use snl::GameSocket;

#[derive(Resource)]
pub struct NetworkManager {
    socket: Option<GameSocket>,
}

impl NetworkManager {
    pub fn new(addr: &str) -> Self {
        let socket = GameSocket::new(addr);

        match socket {
            Ok(socket) => {
                println!("Server ready on address: {}", addr);
                Self {
                    socket: Some(socket),
                }
            }
            Err(_) => Self { socket: None },
        }
    }

    fn handle_helo(&self, addr: String) {
        let mut serializer = Serializer::new(vec![]);
        let _ = &mut serializer << MessageHeader::new(MessageType::Helo, DataType::None).get_data();
        if let Some(socket) = self.socket.as_ref() {
            println!("Send helo to {}", addr);
            socket
                .send(&addr, serializer.get_data())
                .expect("Error Message sending");
        }
    }

    fn handle_hsk(
        &self,
        addr: String,
        mut commands: Commands,
        replication_manager: &mut ReplicationManager,
    ) {
        if let Some(socket) = self.socket.as_ref() {
            let client_net_id = rand::random();
            let connected_client = commands
                .spawn(ConnectedClient {
                    _net_id: client_net_id,
                    address: addr.clone(),
                })
                .id();

            let player_net_id = rand::random();
            let player = ReplicatedNode {
                net_id: player_net_id,
                type_id: 0,
                owner_id: client_net_id,
                x: rand::random_range(-100.0..100.0),
                y: 0.0,
            };

            let player_entity = commands.spawn(player).id();

            let mut client_entity = ClientEntityLink::new(connected_client);

            client_entity
                .possessed_entity
                .insert(player_net_id, player_entity);

            replication_manager
                .client_entities
                .insert(client_net_id, client_entity);

            let mut serializer = Serializer::new(vec![]);
            let _ =
                &mut serializer << MessageHeader::new(MessageType::Hsk, DataType::None).get_data();
            let _ = &mut serializer << client_net_id;
            println!("Send hsk to {}", addr);
            socket
                .send(&addr, serializer.get_data())
                .expect("Error Message sending");
        }
    }

    fn handle_ping(&self, addr: String) {
        let mut serializer = Serializer::new(vec![]);
        let _ = &mut serializer << MessageHeader::new(MessageType::Ping, DataType::None).get_data();

        if let Some(socket) = self.socket.as_ref() {
            socket
                .send(&addr, serializer.get_data())
                .expect("Error Message sending");
        }
    }

    fn handle_bye(
        &self,
        buffer: Vec<u8>,
        mut commands: Commands,
        replication_manager: &mut ReplicationManager,
    ) {
        let mut deserializer = Serializer::new(buffer.clone());
        let mut bb: u32 = 0;

        let _ = &mut deserializer >> &mut bb;

        let mut serializer = Serializer::new(buffer);
        let mut net_id: u32 = 0;
        let _ = &mut serializer >> &mut net_id;

        if let Some(client) = replication_manager.client_entities.get(&net_id) {
            for entity in client.possessed_entity.values() {
                commands.trigger(DestroyEntity { entity: *entity });
            }
            commands.entity(client.client).despawn();
        }
    }

    pub fn poll(
        &self,
        commands: Commands,
        replication_manager: &mut ReplicationManager,
    ) -> Option<(String, MessageHeader, Vec<u8>)> {
        let mut buf = [0; 1500];

        if let Some(socket) = self.socket.as_ref() {
            return match socket.poll(&mut buf) {
                Some((size, socket_addr)) => {
                    let buf = &mut buf[..size];

                    let message_header = MessageHeader::from_data(buf[0]);
                    match message_header.get_message_type() {
                        MessageType::Helo => {
                            self.handle_helo(socket_addr);
                            None
                        }
                        MessageType::Hsk => {
                            self.handle_hsk(socket_addr, commands, replication_manager);
                            None
                        }
                        MessageType::Ping => {
                            self.handle_ping(socket_addr);
                            None
                        }
                        MessageType::Data => Some((socket_addr, message_header, buf[1..].to_vec())),
                        MessageType::Bye => {
                            self.handle_bye(buf[1..].to_vec(), commands, replication_manager);
                            None
                        }
                    }
                }
                None => None,
            };
        }
        None
    }

    pub fn send_data(&self, addr: &String, buffer: &[u8]) {
        if let Some(socket) = self.socket.as_ref() {
            socket.send(&addr, &buffer).expect("Error Message sending");
        }
    }
}
