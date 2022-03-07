use glm::Vec2;
use sfml::graphics::{CircleShape, RenderTarget, Shape, Transformable};

use crate::traits::AsSfmlVector2;

const FRICTION: f32 = 1.;
const SPIN_INIT: f32 = 2.;
const SPIN_FRICTION: f32 = 0.2;
pub struct Boomerang<'a> {
    pos: Vec2,
    vel: Vec2,
    shape: CircleShape<'a>,
    spin: f32
}

impl Boomerang<'_> {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        let mut shape = CircleShape::new(10., 30);
        shape.set_fill_color(sfml::graphics::Color::RED);
        Self { pos, vel, shape ,spin:SPIN_INIT}
    }

    pub fn update(&mut self, dt: f32) {
        self.spin *= 1. - (SPIN_FRICTION * dt);
        self.vel =  self.vel *(1. - (FRICTION * dt));

        let force_dir = glm::mat2(0., -1., 1., 0.) * self.vel;
        self.vel = glm::normalize(self.vel + force_dir * SPIN_INIT * dt) * glm::length(self.vel);
        self.pos = self.pos + self.vel * dt;
    }
    pub fn show(&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.shape
            .set_position((self.pos - self.shape.radius()).as_sfml());
        window.draw(&self.shape);
    }
}
