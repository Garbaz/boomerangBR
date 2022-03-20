use glm::Vec2;
use sfml::graphics::{CircleShape, RenderTarget, RenderWindow, Shape, Transformable};

use crate::{
    resources,
    traits::{AsSfmlVector2, SetRelativeOrigin},
    utils,
};

const FRICTION: f32 = 1.;
const SPIN_INIT: f32 = 2.;
const SPIN_FRICTION: f32 = 0.5;
const RADIUS: f32 = 20.;

const ROATION_SPEED_RATIO: f32 = 800.0;
pub struct Boomerang<'a> {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    shape: CircleShape<'a>,
    pub spin: f32,
}

impl<'a> Boomerang<'a> {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        let mut shape = CircleShape::new(RADIUS, 30);
        shape.set_relative_origin((0.33, 0.5));

        unsafe {
            shape.set_texture(&(&resources::TEXTURES).as_ref().unwrap().boomerang, false);
        }

        Self {
            pos,
            vel,
            radius: RADIUS,
            shape,
            spin: SPIN_INIT,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.spin *= 1. - (SPIN_FRICTION * dt);
        self.vel = self.vel * (1. - (FRICTION * dt));

        let force_dir = glm::mat2(0., -1., 1., 0.) * self.vel;
        self.vel = utils::normalize(self.vel + force_dir * self.spin * dt) * glm::length(self.vel);
        self.pos = self.pos + self.vel * dt;

        self.shape.rotate(self.spin * ROATION_SPEED_RATIO * dt);
    }
    pub fn show(&mut self, window: &mut RenderWindow) {
        self.shape.set_position(self.pos.as_sfml());
        let mut temp = sfml::graphics::CircleShape::new(self.shape.radius(), 30);
        temp.set_relative_origin((0.5, 0.5));
        temp.set_position(self.pos.as_sfml());
        window.draw(&temp);
        window.draw(&self.shape);
    }
}
