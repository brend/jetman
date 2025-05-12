use macroquad::prelude::*;

use jetman::ui::InputState;
use jetman::world::World;

#[macroquad::main("Jetman")]
async fn main() {
    let mut world = World::new();

    loop {
        let input = InputState::from_raylib();
        let fps = 1;
        world.update(&input);
        world.draw();
        visualize_input(&input);
        visualize_fps(fps);
        next_frame().await;
    }
}

fn visualize_input(input: &InputState) {
    let mut y = 10.0;
    let x = 10.0;
    let spacing = 20.0;
    y += spacing;
    draw_text("^=>", x, y, 20.0, if input.thrust { WHITE } else { GRAY });
    y += spacing;
    draw_text("<", x, y, 20.0, if input.turn_left { WHITE } else { GRAY });
    draw_text(
        ">",
        x + 20.0,
        y,
        20.0,
        if input.turn_right { WHITE } else { GRAY },
    );
    y += spacing;
    draw_text("X", x, y, 20.0, if input.sever_link { WHITE } else { GRAY });
}

fn visualize_fps(fps: u32) {
    draw_text(&format!("FPS: {}", fps), 10.0, 10.0, 20.0, WHITE);
}
