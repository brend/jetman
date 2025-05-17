use macroquad::prelude::*;

use jetman::ui::InputState;
use jetman::world::World;

/// Entry point of the jetman application
#[macroquad::main("Jetman")]
async fn main() {
    let mut world = World::new();

    loop {
        let input = InputState::from_raylib();
        world.update(&input);
        world.draw(&input);
        next_frame().await;
    }
}
