use crate::network::connected_client::ConnectedClient;
use crate::replication::events::on_client_connected::ClientConnected;
use crate::replication::events::on_client_disconnected::ClientDisconnected;
use crate::rpc::rpc_manager::RpcReceived;
use bevy::prelude::{Commands, Resource};
use common::message_header::{DataType, MessageHeader, MessageType};
use common::stream_reader::StreamReader;
use common::stream_writer::StreamWriter;
use snl::GameSocket;
use std::time::{SystemTime, UNIX_EPOCH};

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
        let mut stream_writer = StreamWriter::new();
        stream_writer.write_serializable(MessageHeader::init(MessageType::Helo, DataType::None));
        self.send_data(&addr, stream_writer.get_data());
    }

    fn handle_hsk(&self, addr: String, mut commands: Commands) {
        let client_net_id = rand::random();
        let connected_client = commands
            .spawn(ConnectedClient {
                _net_id: client_net_id,
                address: addr.clone(),
            })
            .id();

        commands.trigger(ClientConnected {
            entity: connected_client,
            client_net_id,
        });

        let mut stream_writer = StreamWriter::new();
        stream_writer.write_serializable(MessageHeader::init(MessageType::Hsk, DataType::None));
        stream_writer.write_u32(client_net_id);

        println!("Send hsk to {}", addr);
        self.send_data(&addr, stream_writer.get_data())
    }

    fn handle_ping(&self, addr: String, buffer: &[u8]) {
        let mut stream_writer = StreamWriter::new();
        stream_writer.write_serializable(MessageHeader::init(MessageType::Ping, DataType::None));
        stream_writer.write_bytes(buffer);
        stream_writer.write_u64(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        );

        self.send_data(&addr, stream_writer.get_data())
    }

    fn handle_bye(&self, mut stream_reader: StreamReader, mut commands: Commands) {
        let net_id = stream_reader.read_u32();

        commands.trigger(ClientDisconnected {
            client_net_id: net_id,
        });
    }

    pub fn poll(&self, mut commands: Commands) {
        let mut buf = [0; 1500];
        if let Some(socket) = self.socket.as_ref() {
            match socket.poll(&mut buf) {
                Some((size, socket_addr)) => {
                    let buf = &mut buf[..size];
                    let mut stream_reader = StreamReader::new(buf.to_vec());
                    let message_header: MessageHeader = stream_reader.read_serializable();
                    match message_header.message_type {
                        MessageType::Helo => self.handle_helo(socket_addr),
                        MessageType::Hsk => self.handle_hsk(socket_addr, commands),
                        MessageType::Ping => {
                            self.handle_ping(socket_addr, stream_reader.get_rest_buffer())
                        }
                        MessageType::Data => commands.trigger(RpcReceived { stream_reader }),
                        MessageType::Bye => self.handle_bye(stream_reader, commands),
                    }
                }
                None => {}
            };
        }
    }

    pub fn send_data(&self, addr: &String, buffer: &[u8]) {
        if let Some(socket) = self.socket.as_ref() {
            socket.send(&addr, &buffer).expect("Error Message sending");
        }
    }
}
