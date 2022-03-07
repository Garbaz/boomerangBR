use std::{
    io,
    net::{TcpListener, TcpStream},
};

pub struct Server {
    handler: fn(TcpStream),
    tcpListener: TcpListener,
}

impl Server {
    pub fn new(handler: fn(TcpStream)) -> io::Result<Self> {
        Ok(Self {
            handler: handler,
            tcpListener: TcpListener::bind("127.0.0.1:0")?,
        })
    }

    pub fn check(&self) {
        for s in self.tcpListener.incoming() {
            match s {
                Ok(stream) => (self.handler)(stream),
                Err(e) => println!("Server:: Connection failed: {}", e),
            }
        }
    }
}
