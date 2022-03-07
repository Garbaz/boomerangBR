use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum Message {
    Info { msg: String },
    PlayerUpdate { id: u32, pos: (f32, f32), vel: (f32, f32) },
}

pub trait Messenger {
    fn send(&mut self, msg: &Message) -> Result<(), io::Error>;
    fn receive(&mut self) -> Result<Message, io::Error>;
}

impl Messenger for TcpStream {
    fn send(&mut self, msg: &Message) -> Result<(), io::Error> {
        write!(self, "{}", serde_json::to_string(msg)?)?;
        return Ok(());
    }

    fn receive(&mut self) -> Result<Message, io::Error> {
        let mut buf = vec![0u8; 0x1000];
        let n = self.read(&mut buf)?;
        let msg = String::from_utf8(buf).unwrap();
        return Ok(serde_json::from_str(&msg[..n])?);
    }
}
