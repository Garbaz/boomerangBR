use glm::{vec2, Vec2};

pub fn normalize(v: Vec2) -> Vec2 {
    if v == vec2(0., 0.) {
        return vec2(0., 0.);
    } else {
        return glm::normalize(v);
    }
}
