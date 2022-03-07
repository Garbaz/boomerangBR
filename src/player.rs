use glm::{vec2, Vec2};
use sfml::graphics::{RenderTarget, Transformable};

use crate::{boomerang, traits::AsSfmlVector2};

pub struct Player<'a> {
    pos: Vec2,
    vel: Vec2,
    shape: sfml::graphics::CircleShape<'a>,
    boomerangs: Vec<boomerang::Boomerang<'a>>,
}

impl Player<'_> {
    pub fn new(pos_init: Vec2) -> Self {
        Self {
            pos: pos_init,
            vel: vec2(100., 0.),
            shape: sfml::graphics::CircleShape::new(25., 30),
            boomerangs: Vec::new(),
        }
    }
    pub fn update(&mut self, dt: f32) {

        self.pos = self.pos + self.vel * dt;
        for b in &mut self.boomerangs {
            b.update(dt);
        }
    }
    pub fn show(&mut self, window: &mut sfml::graphics::RenderWindow) {
        for b in &mut self.boomerangs {
            b.show(window);
        }
        self.shape
            .set_position((self.pos - self.shape.radius()).as_sfml());
        window.draw(&self.shape);
    }
}
