use crate::connected_client::ConnectedClient;
use crate::message_header::{DataType, MessageHeader, MessageType};
use crate::network_mapping::NetworkMapping;
use crate::replicated_node::ReplicatedNode;
use crate::serializer::Serializer;
use bevy::prelude::{Commands, ResMut, Resource};
use snl::GameSocket;
use std::io;

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
        mut network_mapping: ResMut<NetworkMapping>,
    ) {
        if let Some(socket) = self.socket.as_ref() {
            let net_id = rand::random();
            let connected_client = commands.spawn(ConnectedClient {
                net_id,
                address: addr.clone(),
            });

            network_mapping.map.insert(net_id, connected_client.id());

            let replicated_node = ReplicatedNode {
                net_id: rand::random(),
                class_id: 0,
                x: rand::random_range(-100.0..100.0),
                y: 0.0,
            };

            commands.spawn(replicated_node);

            let mut serializer = Serializer::new(vec![]);
            let _ =
                &mut serializer << MessageHeader::new(MessageType::Hsk, DataType::None).get_data();
            let _ = &mut serializer << net_id;
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

    fn handle_data(
        &self,
        _addr: String,
        message_header: MessageHeader,
        buffer: &[u8],
        network_mapping: ResMut<NetworkMapping>,
    ) {
        match message_header.get_data_type() {
            DataType::Rpc => {
                let mut serializer = Serializer::new(buffer.to_vec());
                let net_id: u32 = 0;
                let class_id: usize = 0;
                let x: f32 = 0.0;
                let y: f32 = 0.0;
                let _ = &mut serializer << net_id;
                let _ = &mut serializer << class_id;
                let _ = &mut serializer << x;
                let _ = &mut serializer << y;

                if let Some(_entity) = network_mapping.map.get(&net_id) {}
            }
            DataType::Replication => {}
            DataType::None => {}
        }
    }

    pub fn poll(
        &self,
        commands: Commands,
        network_mapping: ResMut<NetworkMapping>,
    ) -> io::Result<()> {
        let mut buf = [0; 1500];

        if let Some(socket) = self.socket.as_ref() {
            match socket.poll(&mut buf) {
                Some((size, socket_addr)) => {
                    let buf = &mut buf[..size];

                    let message_header = MessageHeader::from_data(buf[0]);
                    match message_header.get_message_type() {
                        MessageType::Helo => self.handle_helo(socket_addr),
                        MessageType::Hsk => self.handle_hsk(socket_addr, commands, network_mapping),
                        MessageType::Ping => self.handle_ping(socket_addr),
                        MessageType::Data => self.handle_data(
                            socket_addr,
                            message_header,
                            &buf[1..],
                            network_mapping,
                        ),
                        MessageType::Bye => {}
                    }
                }
                None => {}
            }
        }
        Ok(())
    }

    pub fn send_data(&self, addr: &String, buffer: &[u8]) {
        if let Some(socket) = self.socket.as_ref() {
            socket.send(&addr, &buffer).expect("Error Message sending");
        }
    }
}
