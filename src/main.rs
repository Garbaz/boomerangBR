mod boomerang;
mod player;
mod networking;
mod traits;

use glm::vec2;
use player::Player;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::Clock,
    window::{Event, Style},
};

fn main() {
    let mut player = Player::new(vec2(200., 300.));

    let mut window = RenderWindow::new(
        (1280, 720),
        "Boomerang BR",
        Style::CLOSE,
        &Default::default(),
    );

    let mut clock = Clock::start();
    loop {
        let dt = clock.restart().as_seconds();
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

        window.display();
    }
}
