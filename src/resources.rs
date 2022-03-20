use sfml::{graphics::Texture, SfBox};

pub struct TextureStore {
    pub boomerang: SfBox<Texture>,
}

pub static mut TEXTURES: Option<TextureStore> = None;

pub fn load() {
    unsafe {
        TEXTURES = Some(TextureStore {
            boomerang: Texture::from_file("./res/boomerang.png").unwrap(),
        });
    }
}