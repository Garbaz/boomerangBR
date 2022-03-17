use sfml::graphics::RenderWindow;

use crate::{boomerang::Boomerang, player::Player, traits::AsGlmVector2};

pub struct GameState {
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
    pub fn update(&mut self, window: &RenderWindow, dt: f32) {
        self.players[0].update(&mut self.boomerangs, window.mouse_position().as_glm(), dt);

        // for p in &mut self.players {
        //     p.update(self, dt);
        // }
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
