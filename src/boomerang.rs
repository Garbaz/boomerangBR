use glm::Vec2;
use sfml::{
    graphics::{RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2f,
};

use crate::{
    networking::{
        message::{FromMessage, Message},
        netsync::{NetId, NetSync},
    },
    resources,
    traits::{AsGlmVector2, AsSfmlVector2, AsTupel, SetRelativeOrigin},
    utils,
};

const FRICTION: f32 = 1.;
const SPIN_INIT: f32 = 2.;
const SPIN_FRICTION: f32 = 0.5;
const RADIUS: f32 = 20.;

const ROATION_SPEED_RATIO: f32 = 800.0;
pub struct Boomerang<'a> {
    pub net_id: NetId,
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    shape: RectangleShape<'a>,
    pub spin: f32,
}

impl<'a> Boomerang<'a> {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        let mut shape = RectangleShape::with_size(Vector2f::new(2. * RADIUS, 2. * RADIUS));
        shape.set_relative_origin((0.33, 0.5));

        unsafe {
            if let Some(textures) = &resources::TEXTURES {
                shape.set_texture(&textures.boomerang, false);
            }
        }

        Self {
            net_id: NetId::new(None),
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
        window.draw(&self.shape);
    }
}

impl NetSync for Boomerang<'_> {
    fn take_sync(&mut self, msg: &Message) -> bool {
        match msg {
            Message::BoomerangUpdate { id, pos, rotation, vel } if *id == self.net_id.id => {
                if !self.net_id.author {
                    self.pos = pos.as_glm();
                    self.shape.set_rotation(*rotation);
                    self.vel = vel.as_glm();
                }
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    fn give_sync(&self) -> Option<Message> {
        if self.net_id.author {
            Some(Message::BoomerangUpdate {
                id: self.net_id.id,
                pos: self.pos.as_tupel(),
                rotation: self.shape.rotation(),
                vel: self.vel.as_tupel(),
            })
        } else {
            None
        }
    }
}

impl FromMessage for Boomerang<'_> {
    fn from_message(msg: &Message) -> Option<Self> {
        match msg {
            Message::BoomerangUpdate { id, pos, rotation, vel } => {
                let mut b = Boomerang::new(pos.as_glm(), vel.as_glm());
                b.net_id = NetId::new(Some(*id));
                b.shape.set_rotation(*rotation);
                Some(b)
            }
            _ => None,
        }
    }
}
