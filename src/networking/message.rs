use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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
        match self.write_fmt(format_args!(
            "{}",
            serde_json::to_string(msg)?
        )) {
            Ok(_) => {}
            Err(_) => {
                todo!();
            }
        }
        return Ok(());
    }

    fn receive(&mut self) -> Result<Message, io::Error> {
        let mut msg = String::new();
        self.read_to_string(&mut msg)?;
        return Ok(serde_json::from_str(&msg)?);
    }
}
