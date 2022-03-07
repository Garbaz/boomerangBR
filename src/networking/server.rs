use std::{
    io::{self},
    net::{TcpListener, TcpStream},
};

use super::message::{Message, Messenger};

pub struct Server {
    listener: TcpListener,
    clients: Vec<TcpStream>,
}

impl Server {
    pub fn new() -> io::Result<Self> {
        return Ok(Self {
            listener: TcpListener::bind("127.0.0.1:0")?,
            clients: Vec::new(),
        });
    }

    pub fn accept(&mut self) {
        for s in self.listener.incoming() {
            match s {
                Ok(stream) => {
                    self.clients.push(stream);
                }
                Err(e) => eprintln!("Client failed to connect: `{}`", e),
            }
        }
    }

    pub fn receive_all(&mut self) -> Vec<Message> {
        let mut msgs = Vec::new();
        for stream in self.clients.iter_mut() {
            match stream.receive() {
                Ok(msg) => {
                    msgs.push(msg);
                }
                Err(e) => {
                    eprintln!("Failed to receive message: `{}`", e)
                }
            }
        }
        return msgs;
    }

    pub fn send_all(&mut self, message: &Message) {
        for stream in self.clients.iter_mut() {
            match stream.send(&message) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Client failed to send message: `{}`", e)
                }
            }
        }
    }
}
