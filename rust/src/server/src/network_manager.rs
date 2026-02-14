use crate::message_type::{MessageHeader, MessageType};
use snl::GameSocket;
use std::io;

pub struct NetworkManager {
    socket: GameSocket,
}

impl NetworkManager {
    pub fn new(addr: &str) -> io::Result<Self> {
        let socket = GameSocket::new(addr);
        println!("UDP server started on {}", addr);

        match socket {
            Ok(socket) => Ok(Self { socket }),
            Err(error) => Err(io::Error::new(io::ErrorKind::Other, error)),
        }
    }

    fn handle_helo(&self, addr: String) {
        println!("Receive Helo from {addr}");
        let mut helo_buf = [0u8; 1500];
        helo_buf[0] = MessageType::Helo as u8;
        self.socket
            .send(&addr, &helo_buf)
            .expect("Error Message sending");
    }

    fn handle_hsk(&self, addr: String) {
        println!("Receive Ksk");
        let mut hsk_buf = [0u8; 1500];
        hsk_buf[0] = MessageType::Hsk as u8;
        self.socket
            .send(&addr, &hsk_buf)
            .expect("Error Message sending");
    }

    fn handle_ping(&self, addr: String) {
        println!("Receive Ping");
        let mut ping_buf = [0u8; 1500];
        ping_buf[0] = MessageType::Ping as u8;
        self.socket
            .send(&addr, &ping_buf)
            .expect("Error Message sending");
    }

    fn handle_data(&self, _addr: String, message_header: MessageHeader, _buffer: &[u8]) {
        println!("Receive Data");
        if message_header.is_rpc() {
            println!("Receive RPC");
        } else {
            println!("Receive Replication Data");
        }
    }

    pub fn poll(&self) -> io::Result<()> {
        let mut buf = [0; 1500];
        match self.socket.poll(&mut buf) {
            Some((size, socket_addr)) => {
                let buf = &mut buf[..size];

                let message_header = MessageHeader::from_data(buf[0]);
                match message_header.get_message_type() {
                    MessageType::Helo => Self::handle_helo(self, socket_addr),
                    MessageType::Hsk => Self::handle_hsk(self, socket_addr),
                    MessageType::Ping => Self::handle_ping(self, socket_addr),
                    MessageType::Data => {
                        Self::handle_data(self, socket_addr, message_header, &buf[1..])
                    }
                }
            }
            None => {}
        }
        Ok(())
    }
}
