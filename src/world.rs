use macroquad::prelude::*;

use crate::physics::*;
use crate::terrain::{Terrain, check_collision};
use crate::ui::InputState;

fn generate_ground_poly(width: i32, height: i32, segments: usize) -> Vec<Vec2> {
    let base_y = 500.0;
    let mut top = vec![];

    for i in 0..=segments {
        let x = i as f32 * (width as f32 / segments as f32);
        let y = base_y - macroquad::rand::gen_range(0.0, 80.0);
        top.push(Vec2::new(x, y));
    }

    let bottom = (0..=segments)
        .rev()
        .map(|i| {
            let x = i as f32 * (width as f32 / segments as f32);
            Vec2::new(x, height as f32)
        })
        .collect::<Vec<_>>();

    top.extend(bottom);
    top
}

/// The game world containing physics bodies and terrains
pub struct World {
    pub jetman: Jetman,
    items: Vec<Item>,
    teleports: Vec<Teleporter>,
    gravity: Vec2,
    terrain: Vec<Terrain>,
    camera: Camera2D,
}

impl World {
    /// Create a new game world
    pub fn new() -> Self {
        let terrain = vec![Terrain::polygon(generate_ground_poly(
            screen_width() as i32,
            screen_height() as i32,
            12,
        ))];
        let camera = Camera2D {
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            target: vec2(0.0, 0.0),
            ..Default::default()
        };

        World {
            jetman: Jetman::new(),
            items: vec![Item::new(100.0, 200.0)],
            teleports: vec![Teleporter::new(Vec2::new(400.0, 300.0))],
            gravity: Vec2::new(0.0, 0.01),
            terrain,
            camera,
        }
    }

    /// Update the game world
    pub fn update(&mut self, input: &InputState) {
        let dt = get_frame_time() * 20.0;

        if input.thrust {
            self.jetman.apply_thrust();
        }
        if input.turn_left {
            self.jetman.turn_left();
        }
        if input.turn_right {
            self.jetman.turn_right();
        }

        // Apply gravity to Jetman
        self.jetman.apply_force(self.gravity);

        // Check if item has been dropped into teleporter
        if let Some(item_id) = self.jetman.linked_item {
            let item = &mut self.items[item_id.0];
            let mut teleporting = false;
            for teleport in &self.teleports {
                let diff = item.position() - teleport.position;
                let distance = diff.length();
                if distance < 10.0 {
                    item.body_mut().position = Vec2::new(100.0, 200.0);
                    item.clear_forces();
                    teleporting = true;
                    break;
                }
            }
            if teleporting {
                self.jetman.linked_item = None;
                self.items.remove(item_id.0);
            }
        }

        // Check for linking with items
        let jetman_pos = self.jetman.position();
        for (id, item) in self.items.iter_mut().enumerate() {
            let diff = item.position() - jetman_pos;
            let distance = diff.length();
            if distance < self.jetman.link_distance {
                self.jetman.linked_item = Some(ItemId(id));
            }
        }

        // Check for severing link
        if input.sever_link {
            if let Some(item_id) = self.jetman.linked_item {
                self.jetman.linked_item = None;
                self.items[item_id.0].clear_forces();
            }
        }

        // Enforce rigid connection if Jetman is linked to an item
        if let Some(ItemId(id)) = self.jetman.linked_item {
            let item = &mut self.items[id];
            let item_pos = item.position();
            let delta = item_pos - jetman_pos;
            let distance = delta.length();

            let rest_length = self.jetman.link_distance;
            if distance != 0.0 {
                let direction = delta / distance;
                let correction = direction * (distance - rest_length);

                // Calculate correction ratio based on masses
                let total_mass = self.jetman.mass() + item.mass();
                let jetman_ratio = item.mass() / total_mass;
                let item_ratio = self.jetman.mass() / total_mass;

                // Correct positions
                self.jetman.body_mut().position += correction * jetman_ratio;
                item.body_mut().position -= correction * item_ratio;

                // Optional: also correct velocity along the axis to enforce rigid link
                let relative_velocity = item.velocity() - self.jetman.velocity();
                let projected_velocity = relative_velocity.dot(direction);
                let velocity_correction = direction * projected_velocity;

                self.jetman.body_mut().velocity += velocity_correction * jetman_ratio;
                item.body_mut().velocity -= velocity_correction * item_ratio;
            }
        }

        // Update physics
        self.jetman.update(dt);
        for item in self.items.iter_mut() {
            item.update(dt);
        }

        // Check for terrain collisions
        for terrain in &self.terrain {
            check_collision(&mut self.jetman.body, terrain);
            for item in &mut self.items {
                check_collision(&mut item.body, terrain);
            }
        }

        // center the camera on the jet pod
        let jetman_position = self.jetman_position();
        self.camera.target.x = jetman_position.x;
        self.camera.target.y = jetman_position.y;
        set_camera(&self.camera);
    }

    /// Draw the game world
    pub fn draw(&self, input: &InputState) {
        // clear the screen
        clear_background(BLACK);

        // draw the terrain
        for terrain in &self.terrain {
            terrain.draw();
        }
        // draw the teleporters
        for teleport in &self.teleports {
            teleport.draw();
        }
        // draw the items
        for item in &self.items {
            item.draw();
        }
        // draw the Jetman
        self.jetman.draw();
        // draw the link between Jetman and the item he's linked with
        if let Some(item_id) = self.jetman.linked_item {
            let item = &self.items[item_id.0];
            let jp = self.jetman.position();
            let ip = item.position();
            draw_line(jp.x, jp.y, ip.x, ip.y, 3.0, GREEN);
        }

        // draw thw HUD
        set_default_camera();
        visualize_input(input, &self.jetman);
    }

    pub fn jetman_position(&self) -> Vec2 {
        self.jetman.position()
    }
}

impl Default for World {
    /// Create a game world instance using default values
    fn default() -> Self {
        World::new()
    }
}

/// Draw an HUD visualizing user input
fn visualize_input(input: &InputState, jetman: &Jetman) {
    let mut y = 10.0;
    let x = 10.0;
    let spacing = 20.0;
    y += spacing;
    draw_text("Press W for", x, y, 20.0, GRAY);
    draw_text(
        "THRUST",
        x + 100.0,
        y,
        20.0,
        if input.thrust { WHITE } else { GRAY },
    );
    y += spacing;
    draw_text("Press A to turn     , D to turn", x, y, 20.0, GRAY);
    draw_text(
        "LEFT",
        x + 140.0,
        y,
        20.0,
        if input.turn_left { WHITE } else { GRAY },
    );
    draw_text(
        "RIGHT",
        x + 280.0,
        y,
        20.0,
        if input.turn_right { WHITE } else { GRAY },
    );

    y += spacing;
    if jetman.linked_item.is_some() {
        draw_text("Press S to sever the tractor beam", x, y, 20.0, WHITE);
    }
}
