#![allow(dead_code)]

use boomerang_br::{boomerang::Boomerang, game_state::GameState, player::Player, traits::AsGlmVector2};
use glm::vec2;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, Transformable},
    system::Clock,
    window::{Event, Key, Style},
};

fn main() {
    let mut window = RenderWindow::new(
        (1280, 720),
        "Boomerang BR",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let font = sfml::graphics::Font::from_file("./res/ProcessingSansPro-Semibold.ttf").unwrap();
    let mut text = sfml::graphics::Text::new("", &font, 20);
    text.set_position((10., 10.));
    text.set_fill_color(Color::BLACK);

    let mut game_state = GameState::new();
    game_state
        .players
        .push(Player::new(window.size().as_glm() / 2.));
    game_state
        .boomerangs
        .push(Boomerang::new(
            window.size().as_glm() / 2.,
            vec2(500., 200.),
        ));
    let mut clock = Clock::start();
    loop {
        let dt = clock.restart().as_seconds();

        let mut debug_string = String::new();

        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed | Event::KeyPressed { code: Key::ESCAPE, .. } => return,
                _ => {}
            }
        }

        // UPDATE UPDATE UPDATE
        game_state.update(dt);

        window.clear(Color::rgb(0xCC, 0xCC, 0xCC));
        // DRAW DRAW DRAW
        game_state.show(&mut window);

        debug_string += &format!("DeltaTime: {}\n", dt);
        debug_string += &format!("PlayerPos: {:?}\n", game_state.players[0].pos);

        text.set_string(&debug_string);
        window.draw(&text);

        window.display();
    }
}
