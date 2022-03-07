use sfml::graphics::RenderWindow;

use crate::{boomerang::Boomerang, input::Input, player::Player};

pub struct GameState{
    pub players: Vec<Player>,
    pub boomerangs: Vec<Boomerang>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: Vec::<Player>::new(),
            boomerangs: Vec::<Boomerang>::new(),
        }
    }
    pub fn update(&mut self, dt: f32, kb: &Input) {
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
