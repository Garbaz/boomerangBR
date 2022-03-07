use std::{io, net::{TcpStream, SocketAddr}};

use super::message::{Message, Messenger};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(address: SocketAddr) -> Result<Self, io::Error> {
        let client = Self {
            stream: TcpStream::connect(address)?,
        };
        // client.stream.set_nonblocking(true)?;
        return Ok(client);
    }

    pub fn receive(&mut self) -> Option<Message> {
        match self.stream.receive() {
            Ok(msg) => {
                return Some(msg);
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => {
                eprintln!("Client failed to receive message: `{}`", e)
            }
        }
        return None;
    }

    pub fn send(&mut self, message: &Message) {
        match self.stream.send(&message) {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => {
                eprintln!("Client failed to send message: `{}`", e);
            }
        }
    }
}
