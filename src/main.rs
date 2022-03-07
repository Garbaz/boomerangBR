#![allow(dead_code)]

mod boomerang;
mod game_state;
mod input;
mod networking;
mod player;
mod traits;
mod utils;

use boomerang::Boomerang;
use game_state::GameState;
use glm::vec2;
use input::Input;
use player::Player;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, Transformable},
    system::Clock,
    window::{Event, Style},
};
use traits::AsGlmVector2;

fn main() {
    let mut window = RenderWindow::new(
        (1280, 720),
        "Boomerang BR",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    
    let font = sfml::graphics::Font::from_file("./res/ProcessingSansPro-Semibold.ttf").unwrap();
    let mut text = sfml::graphics::Text::new("test \ntest2", &font, 20);
    text.set_position((10.,10.));
    text.set_fill_color(Color::BLACK);

    let mut keyboard = Input::new();

    let mut game_state = GameState::new();
    game_state
        .players
        .push(Player::new(window.size().as_glm() / 2.));
    game_state.boomerangs.push(Boomerang::new(window.size().as_glm() / 2., vec2(500., 200.)));
    let mut clock = Clock::start();
    loop {
        let dt = clock.restart().as_seconds();

        let mut debug_string = String::new();

        while let Some(ev) = window.poll_event() {
            keyboard.update(ev);
            match ev {
                Event::Closed => return,
                _ => {}
            }
        }
        // UPDATE UPDATE UPDATE
        game_state.update(dt, &keyboard);

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
