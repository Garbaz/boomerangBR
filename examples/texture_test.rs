use boomerang_br::traits::AsSfmlVector2;
use sfml::{
    graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Texture, Transformable},
    system::Clock,
    window::{Key, Style},
};

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const WINDOW_SIZE: (u32, u32) = (WINDOW_WIDTH, WINDOW_HEIGHT);

fn main() {
    let tex = Texture::from_file("./res/boomerang.png").unwrap();

    let boomerang_size = 50.;
    let mut boomerang = CircleShape::new(0.5 * boomerang_size, 32);
    boomerang.set_texture(&tex, false);
    boomerang.set_origin((0.33 * boomerang_size, 0.5 * boomerang_size));

    let mut center_point = CircleShape::new(4., 8);
    center_point.set_fill_color(Color::RED);
    center_point.set_position((
        0.5 * (WINDOW_WIDTH as f32) - center_point.radius(),
        0.5 * (WINDOW_HEIGHT as f32) - center_point.radius(),
    ));

    let mut window = RenderWindow::new(
        WINDOW_SIZE,
        "texture test",
        Style::TITLEBAR,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut clock = Clock::start();
    let mut spin = 0.;
    loop {
        let dt = clock.restart().as_seconds();
        if Key::is_pressed(Key::ESCAPE) {
            return;
        }

        boomerang.rotate(5.0 * 360.0 * dt);

        spin += dt;
        let (sindt,cosdt) = (2.0 * spin).sin_cos();
        boomerang.set_position((0.5 * (WINDOW_WIDTH as f32) + 300. * cosdt, 0.5 * (WINDOW_HEIGHT as f32) + 300. * sindt));

        window.clear(Color::GREEN);
        window.draw(&boomerang);
        window.draw(&center_point);
        window.display();
    }
}
