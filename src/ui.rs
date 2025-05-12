use macroquad::input::{KeyCode, is_key_down, is_key_pressed};

/// The state of the player's input.
pub struct InputState {
    /// Whether the player is thrusting.
    pub thrust: bool,
    /// Whether the player is turning left.
    pub turn_left: bool,
    /// Whether the player is turning right.
    pub turn_right: bool,
    /// Whether the player is severing the link between Jetman and the Item he's linked with.
    pub sever_link: bool,
}

impl InputState {
    /// Create an `InputState` from the current state of the keyboard.
    pub fn from_raylib() -> Self {
        InputState {
            thrust: is_key_down(KeyCode::Up) || is_key_down(KeyCode::W),
            turn_left: is_key_down(KeyCode::Left) || is_key_down(KeyCode::A),
            turn_right: is_key_down(KeyCode::Right) || is_key_down(KeyCode::D),
            sever_link: is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::S),
        }
    }
}
