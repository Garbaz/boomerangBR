use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use rand::Rng;
use serde::{Deserialize, Serialize};

const MAX_MSGS_PER_RECEIVE : usize = 16;

#[derive(Deserialize, Serialize, Debug)]
pub enum Message {
    Info {
        msg: String,
    },
    PlayerUpdate {
        id: u32,
        pos: (f32, f32),
        rotation: f32,
        vel: (f32, f32),
    },
    BoomerangUpdate {
        id: u32,
        pos: (f32, f32),
        rotation: f32,
        vel: (f32, f32),
    },
}

pub trait Messenger {
    fn send(&mut self, msg: &Message) -> Result<(), io::Error>;
    fn receive(&mut self) -> Result<Vec<Message>, io::Error>;
}

impl Messenger for TcpStream {
    fn send(&mut self, msg: &Message) -> Result<(), io::Error> {
        write!(self, "{}|", serde_json::to_string(msg)?)?;
        return Ok(());
    }

    fn receive(&mut self) -> Result<Vec<Message>, io::Error> {
        let mut msgs = Vec::new();
        let mut buf = vec![0u8; 0x10000];
        let n = self.read(&mut buf)?;
        let buf_str = String::from_utf8(buf).unwrap();
        for s in buf_str[..n].split('|') {
            if let Ok(m) = serde_json::from_str(s) {
                msgs.push(m);
                
            } else if s.len() > 0 {
                eprintln!("(!)Bad msg: |{}|", s);
            }
        }
        return Ok(msgs);
    }
}

pub trait FromMessage
where
    Self: Sized,
{
    fn from_message(msg: &Message) -> Option<Self>;
}
