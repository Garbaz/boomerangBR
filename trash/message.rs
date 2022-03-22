use std::{
    collections::VecDeque,
    io::{self, Read, Write},
    net::TcpStream,
};

use serde::{Deserialize, Serialize};

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

struct MessageQueue<'a> {
    tcp_stream: &'a mut TcpStream,
    buffer: Vec<u8>,
    remnant: String,
    messages: VecDeque<Message>,
}

pub trait Messenger {
    fn send(&mut self, msg: &Message) -> Result<(), io::Error>;
    fn receive(&mut self) -> MessageQueue;
}

impl<'a> Iterator for MessageQueue<'a> {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(msg) = self.messages.pop_front() {
            Some(msg)
        } else {
            let n = self.tcp_stream.read(&mut self.buffer).ok()?;
            let buf_str = String::from_utf8(self.buffer).ok()?;
            for s in buf_str[..n].split('|') {
                if let Ok(m) = serde_json::from_str(s) {
                    println!("Good msg: |{}|", s);
                    self.messages.push_back(m);
                } else if s.len() > 0 {
                    self.remnant = s.to_string();
                }
            }
            self.messages.pop_front()
        }
    }
}

impl Messenger for TcpStream {
    fn send(&mut self, msg: &Message) -> Result<(), io::Error> {
        write!(self, "{}|", serde_json::to_string(msg)?)?;
        return Ok(());
    }

    fn receive(&mut self) -> MessageQueue {
        MessageQueue {
            tcp_stream: self,
            buffer: vec![0u8; 0x1000],
            remnant: "".to_string(),
            messages: VecDeque::new(),
        }
    }
}

pub trait FromMessage
where
    Self: Sized,
{
    fn from_message(msg: &Message) -> Option<Self>;
}
