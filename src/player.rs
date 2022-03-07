use glm::{vec2, Vec2};
use sfml::{
    graphics::{RenderTarget, Shape, Transformable},
    window::Key,
};

use crate::{traits::AsSfmlVector2};

const SPEED: f32 = 400.;

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    shape: sfml::graphics::CircleShape<'static>,
}

impl Player{
    pub fn new(pos: Vec2) -> Self {
        let mut shape = sfml::graphics::CircleShape::new(20., 30);
        shape.set_fill_color(sfml::graphics::Color::BLACK);
        Self {
            pos,
            vel: vec2(0., 0.),
            shape,
        }
    }
    pub fn update(&mut self, dt: f32) {
        let key_dir = Player::get_key_dir();
        self.vel = key_dir * SPEED;
        self.pos = self.pos + self.vel * dt;
    }

    pub fn show(&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.shape
            .set_position((self.pos - self.shape.radius()).as_sfml());
        window.draw(&self.shape);
    }
    fn get_key_dir() -> Vec2 {
        let right = Key::is_pressed(Key::RIGHT) | Key::is_pressed(Key::D);
        let left = Key::is_pressed(Key::LEFT) | Key::is_pressed(Key::A);
        let up = Key::is_pressed(Key::UP) | Key::is_pressed(Key::W);
        let down = Key::is_pressed(Key::DOWN) | Key::is_pressed(Key::S);
        let dir = vec2(
            (right as i32 as f32) - (left as i32 as f32),
            (down as i32 as f32) - (up as i32 as f32),
        );
        if dir != vec2(0., 0.) {
            return glm::normalize(dir);
        } else {
            return vec2(0., 0.);
        }
    }
}
