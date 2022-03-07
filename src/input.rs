use std::collections::HashMap;

use sfml::window::{Event, Key};

pub struct Input {
    keys: HashMap<Key, bool>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }
    pub fn update(&mut self, event: Event) {
        match event {
            Event::KeyPressed { code, .. } => {
                self.keys.insert(code, true);
            }
            Event::KeyReleased { code, .. } => {
                self.keys.insert(code, false);
            }

            _ => {}
        }
    }
    pub fn is_key_pressed(&self, key: Key) -> bool {
        if let Some(b) = self.keys.get(&key) {
            return *b;
        } else {
            return false;
        }
    }
}
