use std::collections::HashMap;

use sfml::window::{mouse::Button, Event, Key};

static mut INPUT_STATE: Option<InputState> = None;

#[derive(PartialEq, Eq, Hash, Clone)]
enum HashableButton {
    Left,
    Right,
    Middle,
    XButton1,
    XButton2,
}

impl From<Button> for HashableButton {
    fn from(b: Button) -> Self {
        match b {
            Button::LEFT => HashableButton::Left,
            Button::RIGHT => HashableButton::Right,
            Button::MIDDLE => HashableButton::Middle,
            Button::X_BUTTON_1 => HashableButton::XButton1,
            Button::X_BUTTON_2 => HashableButton::XButton2,
            _ => HashableButton::Left,
        }
    }
}

#[derive(Clone)]
struct InputState {
    // key_state: HashMap<Key, bool>,
    key_state_delta: HashMap<Key, bool>,
    // mouse_state: HashMap<HashableButton, bool>,
    mouse_state_delta: HashMap<HashableButton, bool>,
    mouse_wheel_state: f32,
    mouse_wheel_state_delta: f32,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            // key_state: HashMap::new(),
            key_state_delta: HashMap::new(),
            // mouse_state: HashMap::new(),
            mouse_state_delta: HashMap::new(),
            mouse_wheel_state: 0.,
            mouse_wheel_state_delta: 0.,
        }
    }
}

pub fn clear() {
    unsafe {
        if let Some(is) = &mut INPUT_STATE {
            is.key_state_delta.clear();
            is.mouse_state_delta.clear();
            is.mouse_wheel_state_delta = 0.;
        }
    }
}

pub fn handle_event(event: Event) {
    unsafe {
        if let None = INPUT_STATE {
            INPUT_STATE = Some(InputState::new());
        };

        if let Some(is) = &mut INPUT_STATE {
            match event {
                Event::KeyPressed { code, .. } => {
                    // is.key_state.insert(code, true);
                    is.key_state_delta.insert(code, true);
                }
                Event::KeyReleased { code, .. } => {
                    // is.key_state.insert(code, false);
                    is.key_state_delta.insert(code, false);
                }
                Event::MouseButtonPressed { button, .. } => {
                    // is.mouse_state.insert(button.into(), true);
                    is.mouse_state_delta.insert(button.into(), true);
                }
                Event::MouseButtonReleased { button, .. } => {
                    // is.mouse_state.insert(button.into(), false);
                    is.mouse_state_delta.insert(button.into(), false);
                }
                Event::MouseWheelScrolled { wheel: _, delta, .. } => {
                    is.mouse_wheel_state += delta;
                    is.mouse_wheel_state_delta = delta;
                }
                _ => {}
            };
        }
    }
}

pub fn key_pressed(key: Key) -> bool {
    // unsafe {
    //     if let Some(is) = &INPUT_STATE {
    //         match is.key_state.get(&key) {
    //             Some(true) => true,
    //             _ => false,
    //         }
    //     } else {
    //         false
    //     }
    // }
    key.is_pressed()
}

pub fn key_just_pressed(key: Key) -> bool {
    unsafe {
        if let Some(is) = &INPUT_STATE {
            match is.key_state_delta.get(&key) {
                Some(true) => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

pub fn mouse_pressed(key: Button) -> bool {
    // unsafe {
    //     if let Some(is) = &INPUT_STATE {
    //         match is.mouse_state.get(&key.into()) {
    //             Some(true) => true,
    //             _ => false,
    //         }
    //     } else {
    //         false
    //     }
    // }
    key.is_pressed()
}

pub fn mouse_just_pressed(key: Button) -> bool {
    unsafe {
        if let Some(is) = &INPUT_STATE {
            match is.mouse_state_delta.get(&key.into()) {
                Some(true) => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

pub fn mouse_wheel() -> f32 {
    unsafe {
        if let Some(is) = &INPUT_STATE {
            is.mouse_wheel_state
        } else {
            0.
        }
    }
}

pub fn mouse_wheel_delta() -> f32 {
    unsafe {
        if let Some(is) = &INPUT_STATE {
            is.mouse_wheel_state_delta
        } else {
            0.
        }
    }
}
