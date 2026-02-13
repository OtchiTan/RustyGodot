use crate::message_type::MessageType;
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
        println!("Receive Helo");
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

    fn handle_data(&self, addr: String, buffer: &[u8]) {
        println!("Receive Data");
        match str::from_utf8(buffer) {
            Ok(s) => {
                println!("{}", s);
            }
            Err(e) => {
                eprintln!("Invalid UTF-8 sequence: {}", e);
            }
        }

        let mut response_data = [0u8; 1500];
        response_data[0] = MessageType::Data as u8;
        self.socket
            .send(&addr, &response_data)
            .expect("Error Message sending");
    }

    pub fn poll(&self) -> io::Result<()> {
        let mut buf = [0u8; 1500];

        let buffer = &mut [];
        match self.socket.poll(buffer) {
            Some((size, socket_addr)) => {
                println!("Received message");

                let buf = &mut buf[..size];

                let parsed_message = MessageType::try_from(buf[0]);
                match parsed_message {
                    Ok(v) => match v {
                        MessageType::Helo => Self::handle_helo(self, socket_addr),
                        MessageType::Hsk => Self::handle_hsk(self, socket_addr),
                        MessageType::Ping => Self::handle_ping(self, socket_addr),
                        MessageType::Data => Self::handle_data(self, socket_addr, buf),
                    },
                    Err(err) => {
                        println!("Error: {:?}", err);
                    }
                }
            }
            None => {}
        }
        Ok(())
    }
}
