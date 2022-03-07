mod player;
mod networking;

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

        window.clear(Color::rgb(0xCC, 0xFF, 0xCC));
        // DRAW DRAW DRAW
        window.display();
    }
}
