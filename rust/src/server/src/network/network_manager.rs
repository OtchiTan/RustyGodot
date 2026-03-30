use crate::network::connected_client::ConnectedClient;
use crate::network::{PingReceived, PollEvent};
use crate::replication::events::on_client_connected::ClientConnected;
use crate::replication::events::on_client_disconnected::ClientDisconnected;
use crate::rpc::rpc_manager::InputReceived;
use bevy::prelude::{Resource, World};
use common::message_header::{DataType, MessageHeader, MessageType};
use common::stream_reader::StreamReader;
use common::stream_writer::StreamWriter;
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

    fn handle_helo(&self, addr: String) -> PollEvent {
        let mut stream_writer = StreamWriter::new();
        stream_writer.write_serializable(MessageHeader::init(MessageType::Helo, DataType::None));
        self.send_data(&addr, stream_writer.get_data());
        PollEvent::None
    }

    fn handle_hsk(&self, addr: String, world: &mut World) -> PollEvent {
        let client_net_id = rand::random();
        let connected_client = world
            .spawn(ConnectedClient {
                net_id: client_net_id,
                address: addr.clone(),
                latest_ping: 0,
            })
            .id();

        let client_connected = ClientConnected {
            entity: connected_client,
            client_net_id,
        };

        let mut stream_writer = StreamWriter::new();
        stream_writer.write_serializable(MessageHeader::init(MessageType::Hsk, DataType::None));
        stream_writer.write_u32(client_net_id);

        println!("Send hsk to {}", addr);
        self.send_data(&addr, stream_writer.get_data());
        PollEvent::Connected(client_connected)
    }

    fn handle_ping(&self, address: String, mut stream_reader: StreamReader) -> PollEvent {
        let client_time = stream_reader.read_u64();

        PollEvent::Ping(PingReceived {
            client_time,
            address,
        })
    }

    fn handle_data(&self, stream_reader: StreamReader, message_header: MessageHeader) -> PollEvent {
        if message_header.data_type == DataType::Rpc {
            return PollEvent::Input(InputReceived { stream_reader });
        }
        PollEvent::None
    }

    fn handle_bye(&self, mut stream_reader: StreamReader) -> PollEvent {
        let net_id = stream_reader.read_u32();

        PollEvent::Disconnected(ClientDisconnected {
            client_net_id: net_id,
        })
    }

    pub fn poll(&self, world: &mut World) -> PollEvent {
        let mut buf = [0; 1500];
        if let Some(socket) = self.socket.as_ref() {
            if let Some((size, socket_addr)) = socket.poll(&mut buf) {
                let buf = &mut buf[..size];
                let mut stream_reader = StreamReader::new(buf.to_vec());
                let message_header: MessageHeader = stream_reader.read_serializable();
                return match message_header.message_type {
                    MessageType::Helo => self.handle_helo(socket_addr),
                    MessageType::Hsk => self.handle_hsk(socket_addr, world),
                    MessageType::Ping => self.handle_ping(socket_addr, stream_reader),
                    MessageType::Data => self.handle_data(stream_reader, message_header),
                    MessageType::Bye => self.handle_bye(stream_reader),
                };
            }
        }

        PollEvent::None
    }

    pub fn send_data(&self, addr: &String, buffer: &[u8]) {
        if let Some(socket) = self.socket.as_ref() {
            socket.send(&addr, &buffer).expect("Error Message sending");
        }
    }
}
