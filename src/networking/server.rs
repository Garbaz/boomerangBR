use std::{
    io::{self},
    net::{SocketAddr, TcpListener, TcpStream},
};

use super::message::{Message, Messenger};

pub struct Server {
    listener: TcpListener,
    clients: Vec<(TcpStream, bool)>,
}

impl Server {
    pub fn new(port : &str) -> io::Result<Self> {
        let server = Self {
            listener: TcpListener::bind("127.0.0.1:".to_string() + port)?,
            clients: Vec::new(),
        };
        server.listener.set_nonblocking(true)?;
        return Ok(server);
    }

    pub fn accept(&mut self) {
        match self.listener.accept() {
            Ok((stream, _)) => {
                println!(
                    "Server accepted new connection from {}",
                    stream.peer_addr().unwrap()
                );
                stream.set_nonblocking(true).unwrap();
                self.clients.push((stream, true));
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => {
                eprintln!("Server failed to accept connection: `{}`", e)
            }
        }
    }

    pub fn receive_all(&mut self) -> Vec<Message> {
        self.clients.retain(|(_, alive)| *alive);

        let mut msgs = Vec::new();
        for (stream, alive) in self.clients.iter_mut() {
            match stream.receive() {
                Ok(m) => {
                    msgs.extend(m);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) if e.kind() == io::ErrorKind::BrokenPipe => {
                    *alive = false;
                }
                Err(e) => {
                    eprintln!("Server failed to receive message: `{:?}`:`{}`", e.kind(), e);
                }
            }
        }
        return msgs;
    }

    pub fn send_all(&mut self, msg: &Message) {
        self.clients.retain(|(_, alive)| *alive);

        for (stream, alive) in self.clients.iter_mut() {
            match stream.send(&msg) {
                Ok(_) => {}
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) if e.kind() == io::ErrorKind::BrokenPipe => {
                    *alive = false;
                }
                Err(e) => {
                    eprintln!("Server failed to send message: `{:?}`:`{}`", e.kind(), e)
                }
            }
        }
    }

    pub fn address(&self) -> Option<SocketAddr> {
        match self.listener.local_addr() {
            Ok(address) => Some(address),
            Err(_) => None,
        }
    }
}
