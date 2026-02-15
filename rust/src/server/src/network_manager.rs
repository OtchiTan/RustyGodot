use crate::connected_client::ConnectedClient;
use crate::message_header::{DataType, MessageHeader, MessageType};
use crate::network_mapping::NetworkMapping;
use crate::replicated_node::ReplicatedNode;
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
        let mut helo_buf = [0u8; 1];
        helo_buf[0] = MessageHeader::new(MessageType::Helo, DataType::None).get_data();
        if let Some(socket) = self.socket.as_ref() {
            println!("Send helo to {}", addr);
            socket
                .send(&addr, &helo_buf)
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
                x: 0.0,
                y: 0.0,
            };

            commands.spawn(replicated_node);

            let mut hsk_buf = [0u8; 5];
            hsk_buf[0] = MessageHeader::new(MessageType::Hsk, DataType::None).get_data();
            hsk_buf[1..5].copy_from_slice(&net_id.to_le_bytes());
            println!("Send hsk to {}", addr);
            socket.send(&addr, &hsk_buf).expect("Error Message sending");
        }
    }

    fn handle_ping(&self, addr: String) {
        let mut ping_buf = [0u8; 1500];
        ping_buf[0] = MessageHeader::new(MessageType::Ping, DataType::None).get_data();

        if let Some(socket) = self.socket.as_ref() {
            socket
                .send(&addr, &ping_buf)
                .expect("Error Message sending");
        }
    }

    fn handle_data(
        &self,
        _addr: String,
        message_header: MessageHeader,
        _buffer: &[u8],
    ) {
        match message_header.get_data_type() {
            DataType::Rpc => {}
            DataType::Replication => {}
            DataType::Spawn => {}
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

                    println!(
                        "Received message : {:?} | {:?}",
                        message_header.get_message_type(),
                        message_header.get_data_type()
                    );
                    match message_header.get_message_type() {
                        MessageType::Helo => self.handle_helo(socket_addr),
                        MessageType::Hsk => self.handle_hsk(socket_addr, commands, network_mapping),
                        MessageType::Ping => self.handle_ping(socket_addr),
                        MessageType::Data => self.handle_data(
                            socket_addr,
                            message_header,
                            &buf[1..]
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
