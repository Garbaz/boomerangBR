use super::message::Message;

pub struct NetId {
    pub id: u32,
    pub author: bool,
}

impl NetId {
    pub fn new(id: Option<u32>) -> Self {
        Self {
            id: if let Some(id) = id { id } else { rand::random() },
            author: id.is_none(),
        }
    }
}

pub trait NetSync {
    fn take_sync(&mut self, msg: &Message) -> bool;
    fn give_sync(&self) -> Option<Message>;
}
