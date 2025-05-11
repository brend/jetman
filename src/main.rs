use macroquad::prelude::*;

fn vector_from_angle(angle: f32) -> Vec2 {
    Vec2::new(angle.cos(), angle.sin())
}

/// A physics body.
#[derive(Clone, Copy)]
struct Body {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
}

impl Body {
    /// Create a new body.
    fn new(position: Vec2, mass: f32) -> Self {
        Body {
            position,
            velocity: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
            mass,
        }
    }

    /// Apply a force to the body.
    fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }

    /// Clear all forces acting on the body.
    fn clear_forces(&mut self) {
        self.velocity *= 0.0;
        self.acceleration *= 0.0;
    }

    /// Update the body's position based on its velocity and acceleration.
    fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
        self.acceleration = Vec2::ZERO;
    }
}

trait Bodied {
    /// Get a reference to the body.
    fn body(&self) -> &Body;

    /// Get a mutable reference to the body.
    fn body_mut(&mut self) -> &mut Body;

    /// Apply a force to the body.
    fn apply_force(&mut self, force: Vec2) {
        self.body_mut().apply_force(force);
    }

    /// Clear all forces acting on the body.
    fn clear_forces(&mut self) {
        self.body_mut().clear_forces();
    }

    /// Update the body's position based on its velocity and acceleration.
    fn update(&mut self, dt: f32) {
        self.body_mut().update(dt);
    }

    fn position(&self) -> Vec2 {
        self.body().position
    }

    fn velocity(&self) -> Vec2 {
        self.body().velocity
    }

    fn mass(&self) -> f32 {
        self.body().mass
    }
}

#[derive(Clone, Copy)]
struct ItemId(usize);

struct Jetman {
    body: Body,
    heading: f32,
    link_distance: f32,
    linked_item: Option<ItemId>,
    thrusting: i32,
}

impl Jetman {
    fn new() -> Self {
        Jetman {
            body: Body::new(Vec2::new(200.0, 200.0), 1.0),
            heading: 0.0,
            link_distance: 50.0,
            linked_item: None,
            thrusting: 0,
        }
    }

    fn apply_thrust(&mut self) {
        let thrust = vector_from_angle(self.heading) * 0.1;
        self.body.apply_force(thrust);
        self.thrusting = 2;
    }

    fn turn_left(&mut self) {
        self.heading -= 0.1;
    }

    fn turn_right(&mut self) {
        self.heading += 0.1;
    }

    fn update(&mut self, dt: f32) {
        self.body.update(dt);
        self.thrusting -= 1;
    }

    fn draw(&self) {
        let position = self.body.position;
        let dir = vector_from_angle(self.heading);
        let tip = position + dir * 8.0;
        draw_circle(position.x, position.y, 10.0, Color::from_hex(0x807CF4));
        draw_circle_lines(position.x, position.y, 10.0, 1.0, Color::from_hex(0x3524E3));
        draw_ellipse(tip.x, tip.y, 4.0, 4.0, 0.0, WHITE);
        if self.thrusting > 0 {
            // draw an orange flame (an ellipse) at the back of the jetman
            let flame = position - dir * 10.0;
            draw_ellipse(flame.x, flame.y, 4.0, 8.0, 0.0, ORANGE);
        }
    }
}

impl Bodied for Jetman {
    fn body(&self) -> &Body {
        &self.body
    }

    fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }
}

struct Item {
    body: Body,
}

impl Item {
    fn new(x: f32, y: f32) -> Self {
        Item {
            body: Body::new(Vec2::new(x, y), 1.0),
        }
    }

    fn draw(&self) {
        draw_rectangle(
            self.body.position.x - 15.0,
            self.body.position.y - 10.0,
            30.0,
            20.0,
            LIGHTGRAY,
        );
    }
}

impl Bodied for Item {
    fn body(&self) -> &Body {
        &self.body
    }

    fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }
}

/// A teleporter that allows Jetman to drop items.
struct Teleporter {
    position: Vec2,
}

impl Teleporter {
    fn new(position: Vec2) -> Self {
        Teleporter { position }
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 10.0, YELLOW);
    }
}

/// The game world.
struct World {
    jetman: Jetman,
    items: Vec<Item>,
    teleports: Vec<Teleporter>,
    gravity: Vec2,
}

impl World {
    /// Create a new game world.
    fn new() -> Self {
        World {
            jetman: Jetman::new(),
            items: vec![Item::new(100.0, 200.0)],
            teleports: vec![Teleporter::new(Vec2::new(400.0, 300.0))],
            gravity: Vec2::new(0.0, 0.01),
        }
    }

    /// Update the game world.
    fn update(&mut self, input: &InputState) {
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
    }

    /// Draw the game world.
    fn draw(&self) {
        // clear the screen
        clear_background(BLACK);
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
    }
}

/// The state of the player's input.
struct InputState {
    /// Whether the player is thrusting.
    thrust: bool,
    /// Whether the player is turning left.
    turn_left: bool,
    /// Whether the player is turning right.
    turn_right: bool,
    /// Whether the player is severing the link between Jetman and the Item he's linked with.
    sever_link: bool,
}

impl InputState {
    /// Create an `InputState` from the current state of the keyboard.
    fn from_raylib() -> Self {
        InputState {
            thrust: is_key_down(KeyCode::Up) || is_key_down(KeyCode::W),
            turn_left: is_key_down(KeyCode::Left) || is_key_down(KeyCode::A),
            turn_right: is_key_down(KeyCode::Right) || is_key_down(KeyCode::D),
            sever_link: is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::S),
        }
    }
}

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
