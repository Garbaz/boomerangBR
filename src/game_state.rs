use sfml::graphics::RenderWindow;

use crate::{boomerang::Boomerang, keyboard::Keyboard, player::Player};

pub struct GameState<'a> {
    pub players: Vec<Player<'a>>,
    pub boomerangs: Vec<Boomerang<'a>>,
}

impl GameState<'_> {
    pub fn new() -> Self {
        Self {
            players: Vec::<Player>::new(),
            boomerangs: Vec::<Boomerang>::new(),
        }
    }
    pub fn update(&mut self, dt: f32, kb: &Keyboard) {
        for p in &mut self.players {
            p.update(dt, kb);
        }
        for b in &mut self.boomerangs {
            b.update(dt);
        }
    }
    pub fn show(&mut self, window: &mut RenderWindow) {
        for p in &mut self.players {
            p.show(window);
        }
        for b in &mut self.boomerangs {
            b.show(window);
        }
    }
}
