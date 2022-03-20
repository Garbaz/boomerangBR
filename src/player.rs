use glm::{vec2, Vec2};
use sfml::{
    graphics::{RenderTarget, Shape, Transformable},
    window::{mouse::Button, Key},
};

use crate::{
    boomerang::Boomerang,
    input,
    traits::{AsSfmlVector2, SetRelativeOrigin},
    utils,
};

const SPEED: f32 = 400.;
const MAX_THROW_SPEED: f32 = 1000.;
const MAX_THROW_SPEED_DIST: f32 = 250.;
const RADIUS: f32 = 20.;
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    shape: sfml::graphics::CircleShape<'static>,
    pub hp: f32,
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        let mut shape = sfml::graphics::CircleShape::new(RADIUS, 30);
        shape.set_relative_origin((0.5, 0.5));
        shape.set_fill_color(sfml::graphics::Color::BLACK);
        Self {
            pos,
            vel: vec2(0., 0.),
            radius: RADIUS,
            shape,
            hp: 100.,
        }
    }
    pub fn update(&mut self, boomerangs: &mut Vec<Boomerang>, mouse_pos: Vec2, dt: f32) {
        let key_dir = Player::get_key_dir();
        self.vel = key_dir * SPEED;
        self.pos = self.pos + self.vel * dt;
        if input::mouse_just_pressed(Button::LEFT) {
            let mouse_diff = mouse_pos - self.pos;
            let throw_vel = glm::normalize(mouse_diff)
                * glm::min(1., glm::length(mouse_diff) / MAX_THROW_SPEED_DIST)
                * MAX_THROW_SPEED;
            self.throw_boomerang(boomerangs, throw_vel);
        }
        self.boomerang_collision(boomerangs);
    }

    pub fn show(&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.shape.set_position((self.pos).as_sfml());
        window.draw(&self.shape);
    }
    fn get_key_dir() -> Vec2 {
        let right = input::key_pressed(Key::RIGHT) | input::key_pressed(Key::D);
        let left = input::key_pressed(Key::LEFT) | input::key_pressed(Key::A);
        let up = input::key_pressed(Key::UP) | input::key_pressed(Key::W);
        let down = input::key_pressed(Key::DOWN) | input::key_pressed(Key::S);
        let dir = vec2(
            (right as i32 as f32) - (left as i32 as f32),
            (down as i32 as f32) - (up as i32 as f32),
        );
        return utils::normalize(dir);
    }
    fn boomerang_collision(&mut self, boomerangs: &mut Vec<Boomerang>) {
        for b in boomerangs {
            if glm::distance(self.pos, b.pos) <= self.radius + b.radius {
                self.hp -= 10.;
            }
        }
    }

    fn throw_boomerang(&self, boomerangs: &mut Vec<Boomerang>, vel: Vec2) {
        boomerangs.push(Boomerang::new(self.pos, vel));
    }
}
