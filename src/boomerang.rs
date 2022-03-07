use glm::Vec2;
use sfml::graphics::{Transformable, RenderTarget};

use crate::traits::AsSfmlVector2;

pub struct Boomerang<'a> {
    pos: Vec2,
    vel: Vec2,
    shape: sfml::graphics::CircleShape<'a>,
}

impl Boomerang<'_> {
    pub fn update(&mut self, dt: f32) {
        self.pos = self.pos + self.vel * dt;
    }
    pub fn show(&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.shape
            .set_position((self.pos - self.shape.radius()).as_sfml());
        window.draw(&self.shape);
    }
}
