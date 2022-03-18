use sfml::{graphics::{Texture}, SfBox};

pub struct TextureStore {
    pub boomerang: SfBox<Texture>,
}

// pub struct ShapeStore<'a> {
//     boomerang: CircleShape<'a>,
// }

thread_local! {
    pub static TEXTURES : TextureStore = TextureStore {
        boomerang: Texture::from_file("./res/boomerang.png").unwrap(),
    };
}