use glm::{vec2, Vec2};

pub fn normalize(v: Vec2) -> Vec2 {
    let temp = glm::normalize(v);
    if temp.x.is_nan() || temp.y.is_nan() || temp.x.is_infinite() || temp.y.is_infinite(){
        return vec2(0., 0.);
    } else {
        return temp;
    }
}
