use boomerang_br::{boomerang::Boomerang, game_state::GameState, input, player::Player, traits::AsGlmVector2, resources};
use glm::vec2;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, Transformable},
    system::Clock,
    window::{Event, Key, Style, VideoMode},
};

fn main() {
    let mut window = RenderWindow::new(
        // (1280, 720),
        VideoMode::desktop_mode(),
        "Boomerang BR",
        // Style::CLOSE,
        Style::FULLSCREEN,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    resources::load();

    let font = sfml::graphics::Font::from_file("./res/ProcessingSansPro-Semibold.ttf").unwrap();
    let mut text = sfml::graphics::Text::new("", &font, 30);
    text.set_position((10., 10.));
    text.set_fill_color(Color::BLACK);

    let mut game_state = GameState::new();
    game_state
        .players
        .push(Player::new(window.size().as_glm() / 2.));
    game_state.boomerangs.push(Boomerang::new(
        window.size().as_glm() / 2.,
        vec2(500., 200.),
    ));

    let mut clock = Clock::start();
    loop {

        let dt = clock.restart().as_seconds();

        let mut debug_string = String::new();

        input::clear();
        while let Some(ev) = window.poll_event() {
            input::handle_event(ev);
            match ev {
                Event::Closed | Event::KeyPressed { code: Key::ESCAPE, .. } => return,
                _ => {}
            }
        }

        // UPDATE UPDATE UPDATE
        game_state.update(&window, dt);

        window.clear(Color::rgb(0xCC, 0xCC, 0xCC));
        // DRAW DRAW DRAW
        game_state.show(&mut window);

        debug_string += &format!("DeltaTime: {}\n", dt);
        debug_string += &format!("PlayerPos: {:?}\n", game_state.players[0].pos);
        debug_string += &format!("BoomerangsLen: {:?}\n", game_state.boomerangs.len());
        debug_string += &format!("Boomerang0Pos: {:?}\n", game_state.boomerangs[0].pos);
        debug_string += &format!("Boomerang0Vel: {:?}\n", game_state.boomerangs[0].vel);
        debug_string += &format!("Boomerang0Spin: {:?}\n", game_state.boomerangs[0].spin);

        text.set_string(&debug_string);
        window.draw(&text);

        window.display();
    }
}
