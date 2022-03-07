mod boomerang;
mod player;
mod networking;
mod traits;
mod utils;

use glm::vec2;
use player::Player;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::Clock,
    window::{Event, Style},
};

fn main() {

    let mut window = RenderWindow::new(
        (1280, 720),
        "Boomerang BR",
        Style::CLOSE,
        &Default::default(),
    );
    let font = sfml::graphics::Font::from_file("./res/ProcessingSansPro-Semibold.ttf").unwrap();
    let mut text = sfml::graphics::Text::new("test \ntest2", &font, 30);
    text.set_fill_color(Color::BLACK);
    
    let mut player = Player::new(vec2(200., 300.));


    let mut clock = Clock::start();
    loop {
        let dt = clock.restart().as_seconds();

        let mut debug_string = String::new();

        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => return,
                _ => {}
            }
        }

        // UPDATE UPDATE UPDATE
        player.update(dt);

        window.clear(Color::rgb(0xCC, 0xFF, 0xCC));
        // DRAW DRAW DRAW
        player.show(&mut window);


        debug_string += &format!("DeltaTime: {}\n", dt);
        text.set_string(&debug_string);
        window.draw(&text);

        window.display();
    }
}
