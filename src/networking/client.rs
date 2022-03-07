use std::{io, net::TcpStream, ptr::NonNull};

use super::message::{Message, Messenger};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(server: String) -> Result<Self, io::Error> {
        return Ok(Self {
            stream: TcpStream::connect(server)?,
        });
    }

    pub fn receive(&mut self) -> Option<Message> {
        match self.stream.receive() {
            Ok(msg) => {
                return Some(msg);
            }
            Err(e) => {
                eprintln!("Client failed to receive message: `{}`", e)
            }
        }
        return None;
    }

    pub fn send(&mut self, message: &Message) {
        match self.stream.send(&message) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Client failed to send message: `{}`", e);
            }
        }
    }
}
