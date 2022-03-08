use sfml::system::Vector2f;

pub trait AsTupel {
    fn as_tupel(&self) -> (f32, f32);
}

impl AsTupel for glm::Vec2 {
    fn as_tupel(&self) -> (f32, f32) {
        return (self.x, self.y);
    }
}

pub trait AsGlmVector2 {
    fn as_glm(&self) -> glm::Vec2;
}

impl<T> AsGlmVector2 for sfml::system::Vector2<T>
where
    T: Into<f64> + Copy,
{
    fn as_glm(&self) -> glm::Vec2 {
        glm::vec2(self.x.into() as f32, self.y.into() as f32)
    }
}

pub trait AsSfmlVector2 {
    fn as_sfml(&self) -> Vector2f;
}

impl AsSfmlVector2 for glm::Vector2<f32> {
    fn as_sfml(&self) -> Vector2f {
        Vector2f::new(self.x, self.y)
    }
}

impl AsSfmlVector2 for (u32, u32) {
    fn as_sfml(&self) -> Vector2f {
        Vector2f::new(self.0 as f32, self.1 as f32)
    }
}