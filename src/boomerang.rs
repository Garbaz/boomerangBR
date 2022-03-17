use glm::Vec2;
use sfml::graphics::{CircleShape, RenderTarget, Shape, Transformable, Texture};

use crate::traits::AsSfmlVector2;

const FRICTION: f32 = 1.;
const SPIN_INIT: f32 = 2.;
const SPIN_FRICTION: f32 = 0.2;

const ROATION_SPEED_RATIO : f32 = 3.0;
pub struct Boomerang<'a> {
    pos: Vec2,
    vel: Vec2,
    shape: CircleShape<'a>,
    spin: f32
}

impl<'a> Boomerang<'a> {
    pub fn new(pos: Vec2, vel: Vec2, texture : &'a Texture) -> Self {
        let size = 40.;
        let mut shape = CircleShape::new(0.5 * size, 30);
        // shape.set_fill_color(sfml::graphics::Color::RED);
        shape.set_texture(texture, false);
        shape.set_origin((0.33 * size, 0.5 * size));
        Self { pos, vel, shape ,spin:SPIN_INIT}
    }

    pub fn update(&mut self, dt: f32) {
        self.spin *= 1. - (SPIN_FRICTION * dt);
        self.vel =  self.vel *(1. - (FRICTION * dt));

        let force_dir = glm::mat2(0., -1., 1., 0.) * self.vel;
        self.vel = glm::normalize(self.vel + force_dir * SPIN_INIT * dt) * glm::length(self.vel);
        self.pos = self.pos + self.vel * dt;

        self.shape.rotate(glm::length(self.vel) * ROATION_SPEED_RATIO * dt);
    }
    pub fn show(&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.shape
            .set_position((self.pos - self.shape.radius()).as_sfml());
        window.draw(&self.shape);
    }
}
