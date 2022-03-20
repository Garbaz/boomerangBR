use glm::{vec2, Vec2};

pub fn normalize(v: Vec2) -> Vec2 {
    let temp = glm::normalize(v);
    if temp == vec2(f32::NAN, f32::NAN){
        return vec2(0., 0.);
    } else {
        return temp;
    }
}
