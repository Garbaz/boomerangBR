use boomerang_br::{
    boomerang::Boomerang,
    game_state::GameState,
    input,
    networking::{
        client::Client,
        message::{FromMessage, Message},
        netsync::NetSync,
    },
    player::Player,
    resources,
    traits::AsGlmVector2,
};
use glm::vec2;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, Transformable},
    system::Clock,
    window::{Event, Key, Style, VideoMode},
};

const SERVER_ADDR: &str = "127.0.0.1:1729";
const TICK_RATE: f32 = 30.;

fn main() {
    let mut window = RenderWindow::new(
        (960, 540),
        // VideoMode::desktop_mode(),
        "Boomerang BR",
        Style::CLOSE,
        // Style::FULLSCREEN,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    window.set_key_repeat_enabled(false);

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

    let mut client = Client::new(SERVER_ADDR.parse().unwrap());
    if let Err(err) = &client {
        eprintln!("{}", err);
        println!("Couldn't connect to server, is it running? Proceeding offline...");
    }

    let mut clock = Clock::start();
    let mut network_clock = Clock::start();
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

        // NETWORK NETWORK NETWORK
        if let Ok(client) = &mut client {
            if network_clock.elapsed_time().as_seconds() > 1. / TICK_RATE {
                network_clock.restart();
                network_update(client, &mut game_state);
            }
        }

        // UPDATE UPDATE UPDATE
        game_state.update(&window, dt);

        window.clear(Color::rgb(0xCC, 0xCC, 0xCC));
        // DRAW DRAW DRAW
        game_state.show(&mut window);

        debug_string += &format!("DeltaTime: {}\n", dt);
        debug_string += &format!("PlayerPos: {:?}\n", game_state.players[0].pos);
        debug_string += &format!("PlayerHp: {:?}\n", game_state.players[0].hp);
        debug_string += &format!("Input Test: {}\n", input::key_just_pressed(Key::SPACE));
        debug_string += &format!("Focus: {}\n", window.has_focus());

        text.set_string(&debug_string);
        window.draw(&text);

        window.display();
    }
}

fn network_update(client: &mut Client, game_state: &mut GameState) {
    while let Some(msgs) = client.receive() {
        for msg in msgs {
            match msg {
                Message::PlayerUpdate { .. } => {
                    let mut new = true;
                    for p in game_state.players.iter_mut() {
                        if p.take_sync(&msg) {
                            new = false;
                            break;
                        }
                    }
                    if new {
                        if let Some(p) = Player::from_message(&msg) {
                            game_state.players.push(p);
                        }
                    }
                }
                Message::BoomerangUpdate { .. } => {
                    let mut new = true;
                    for p in game_state.boomerangs.iter_mut() {
                        if p.take_sync(&msg) {
                            new = false;
                            break;
                        }
                    }
                    if new {
                        if let Some(b) = Boomerang::from_message(&msg) {
                            game_state.boomerangs.push(b);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    for p in game_state.players.iter() {
        if let Some(msg) = p.give_sync() {
            client.send(&msg);
        }
    }

    for b in game_state.boomerangs.iter() {
        if let Some(msg) = b.give_sync() {
            client.send(&msg);
        }
    }
}
