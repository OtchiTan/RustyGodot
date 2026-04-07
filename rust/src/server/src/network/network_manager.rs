use crate::network::PingReceived;
use crate::network::connected_client::ConnectedClient;
use crate::replication::events::on_client_connected::ClientConnected;
use crate::replication::events::on_client_disconnected::ClientDisconnected;
use bevy::prelude::{Commands, Resource};
use common::input_packet::InputBuffer;
use common::message_header::{DataType, MessageHeader, MessageType};
use common::ping_request::PingRequest;
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

    fn handle_helo(&self, addr: String) {
        let mut stream_writer = StreamWriter::new();
        stream_writer.write_serializable(MessageHeader::init(MessageType::Helo, DataType::None));
        self.send_data(&addr, stream_writer.get_data());
    }

    fn handle_hsk(&self, addr: String, commands: &mut Commands) {
        let client_net_id = rand::random();
        let connected_client = commands
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
        commands.trigger(client_connected);
    }

    fn handle_ping(
        &self,
        address: String,
        mut stream_reader: StreamReader,
        commands: &mut Commands,
    ) {
        let ping_request: PingRequest = stream_reader.read_serializable();

        commands.trigger(PingReceived {
            ping_request,
            address,
        })
    }

    fn handle_bye(&self, mut stream_reader: StreamReader, commands: &mut Commands) {
        let net_id = stream_reader.read_u32();

        commands.trigger(ClientDisconnected {
            client_net_id: net_id,
        })
    }

    pub fn poll(&self, mut commands: Commands) -> Vec<InputBuffer> {
        let mut input_buffers = Vec::new();

        loop {
            let mut buf = [0; 1500];
            if let Some(socket) = self.socket.as_ref() {
                if let Some((size, socket_addr)) = socket.poll(&mut buf) {
                    let buf = &mut buf[..size];
                    let mut stream_reader = StreamReader::new(buf.to_vec());
                    let message_header: MessageHeader = stream_reader.read_serializable();
                    match message_header.message_type {
                        MessageType::Helo => self.handle_helo(socket_addr),
                        MessageType::Hsk => self.handle_hsk(socket_addr, &mut commands),
                        MessageType::Ping => {
                            self.handle_ping(socket_addr, stream_reader, &mut commands)
                        }
                        MessageType::Data => input_buffers.push(stream_reader.read_serializable()),
                        MessageType::Bye => self.handle_bye(stream_reader, &mut commands),
                    };
                } else {
                    break;
                }
            }
        }

        input_buffers
    }

    pub fn send_data(&self, addr: &String, buffer: &[u8]) {
        if let Some(socket) = self.socket.as_ref() {
            socket.send(&addr, &buffer).expect("Error Message sending");
        }
    }
}
