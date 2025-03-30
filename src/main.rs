use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

const TITLE: &str = "Jetman";

fn vector_from_angle(angle: f32) -> Vector2 {
    Vector2::new(angle.cos(), angle.sin())
}

/// A physics body.
#[derive(Clone, Copy)]
struct Body {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    mass: f32,
}

impl Body {
    /// Create a new body.
    fn new(position: Vector2, mass: f32) -> Self {
        Body {
            position,
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            mass,
        }
    }

    /// Apply a force to the body.
    fn apply_force(&mut self, force: Vector2) {
        self.acceleration += force / self.mass;
    }

    /// Clear all forces acting on the body.
    fn clear_forces(&mut self) {
        self.velocity *= 0.0;
        self.acceleration *= 0.0;
    }

    /// Update the body's position based on its velocity and acceleration.
    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }
}

trait Bodied {
    /// Get a reference to the body.
    fn body(&self) -> &Body;

    /// Get a mutable reference to the body.
    fn body_mut(&mut self) -> &mut Body;

    /// Apply a force to the body.
    fn apply_force(&mut self, force: Vector2) {
        self.body_mut().apply_force(force);
    }

    /// Clear all forces acting on the body.
    fn clear_forces(&mut self) {
        self.body_mut().clear_forces();
    }

    /// Update the body's position based on its velocity and acceleration.
    fn update(&mut self) {
        self.body_mut().update();
    }

    fn position(&self) -> Vector2 {
        self.body().position
    }

    fn velocity(&self) -> Vector2 {
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
            body: Body::new(Vector2::new(200.0, 200.0), 1.0),
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

    fn update(&mut self) {
        self.body.update();
        self.thrusting -= 1;
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        let position = self.body.position;
        let dir = vector_from_angle(self.heading);
        let tip = position + dir * 8.0;
        d.draw_circle_v(position, 10.0, Color::from_hex("807CF4").unwrap());
        d.draw_circle_lines(position.x as i32, position.y as i32, 10.0, Color::from_hex("3524E3").unwrap());
        d.draw_ellipse(tip.x as i32, tip.y as i32, 4.0, 4.0, Color::WHITESMOKE);
        if self.thrusting > 0 {
            // draw an orange flame (an ellipse) at the back of the jetman
            let flame = position - dir * 10.0;
            d.draw_ellipse(flame.x as i32, flame.y as i32, 4.0, 8.0, Color::ORANGE);
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
            body: Body::new(Vector2::new(x, y), 1.0),
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.body.position.x as i32 - 15, 
            self.body.position.y as i32 - 10, 
            30, 20, Color::LIGHTGRAY);
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
    position: Vector2,
}

impl Teleporter {
    fn new(position: Vector2) -> Self {
        Teleporter { position }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.position, 10.0, Color::PALEGOLDENROD);
    }
}

/// The game world.
struct World {
    jetman: Jetman,
    items: Vec<Item>,
    teleports: Vec<Teleporter>,
    gravity: Vector2,
}

impl World {
    /// Create a new game world.
    fn new() -> Self {
        World {
            jetman: Jetman::new(),
            items: vec![Item::new(100.0, 200.0)],
            teleports: vec![Teleporter::new(Vector2::new(400.0, 300.0))],
            gravity: Vector2::new(0.0, 0.01),
        }
    }

    /// Update the game world.
    fn update(&mut self, input: &InputState) {
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
                    item.body_mut().position = Vector2::new(100.0, 200.0);
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
        self.jetman.update();
        for item in self.items.iter_mut() {
            item.update();
        }
    }

    /// Draw the game world.
    fn draw(&self, d: &mut RaylibDrawHandle) {
        // clear the screen
        d.clear_background(Color::BLACK);
        // draw the teleporters
        for teleport in &self.teleports {
            teleport.draw(d);
        }
        // draw the items
        for item in &self.items {
            item.draw(d);
        }
        // draw the Jetman
        self.jetman.draw(d);
        // draw the link between Jetman and the item he's linked with
        if let Some(item_id) = self.jetman.linked_item {
            let item = &self.items[item_id.0];
            d.draw_line_ex(self.jetman.position(), item.position(), 3.0, Color::YELLOWGREEN);
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
    fn from_raylib(rl: &RaylibHandle) -> Self {
        InputState {
            thrust: rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W),
            turn_left: rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A),
            turn_right: rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D),
            sever_link: rl.is_key_pressed(KeyboardKey::KEY_S) || rl.is_key_pressed(KeyboardKey::KEY_S),
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title(TITLE)
        .build();

    rl.set_target_fps(30);

    let mut world = World::new();

    while !rl.window_should_close() {
        let input = InputState::from_raylib(&rl);
        let fps = rl.get_fps();
        let mut d = rl.begin_drawing(&thread);
        world.update(&input);
        world.draw(&mut d);
        visualize_input(&input, &mut d);
        visualize_fps(fps, &mut d);
    }
}

fn visualize_input(input: &InputState, d: &mut RaylibDrawHandle) {
    let mut y = 10;
    let x = 10;
    let spacing = 20;
    y += spacing;
    d.draw_text("^=>", x, y, 20, if input.thrust { Color::WHITE } else { Color::GRAY });
    y += spacing;
    d.draw_text("<", x, y, 20, if input.turn_left { Color::WHITE } else { Color::GRAY });
    d.draw_text(">", x + 20, y, 20, if input.turn_right { Color::WHITE } else { Color::GRAY });
    y += spacing;
    d.draw_text("X", x, y, 20, if input.sever_link { Color::WHITE } else { Color::GRAY });
}

fn visualize_fps(fps: u32, d: &mut RaylibDrawHandle) {
    d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::WHITE);
}