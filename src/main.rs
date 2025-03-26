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

    /// Get the position of the body.
    fn position(&self) -> Vector2 {
        self.body().position
    }

    /// Get the velocity of the body.
    fn velocity(&self) -> Vector2 {
        self.body().velocity
    }

    /// Get the acceleration of the body.
    fn acceleration(&self) -> Vector2 {
        self.body().acceleration
    }

    /// Get the mass of the body.
    fn mass(&self) -> f32 {
        self.body().mass
    }

    /// Apply a force to the body.
    fn apply_force(&mut self, force: Vector2) {
        self.body_mut().apply_force(force);
    }

    /// Update the body's position based on its velocity and acceleration.
    fn update(&mut self) {
        self.body_mut().update();
    }
}

#[derive(Clone, Copy)]
struct ItemId(usize);

struct Jetman {
    body: Body,
    heading: f32,
    linked_item: Option<ItemId>,
}

impl Jetman {
    fn new() -> Self {
        Jetman {
            body: Body::new(Vector2::new(200.0, 200.0), 1.0),
            heading: 0.0,
            linked_item: None,
        }
    }

    fn apply_thrust(&mut self) {
        let thrust = vector_from_angle(self.heading) * 0.1;
        self.body.apply_force(thrust);
    }

    fn turn_left(&mut self) {
        self.heading -= 0.1;
    }

    fn turn_right(&mut self) {
        self.heading += 0.1;
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        let position = self.body.position;
        d.draw_circle_v(position, 10.0, Color::LIGHTBLUE);
        d.draw_circle_lines(position.x as i32, position.y as i32, 10.0, Color::BLUE);
        let tip = position + vector_from_angle(self.heading) * 20.0;
        d.draw_line_v(position, tip, Color::RED);
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
        d.draw_rectangle(self.body.position.x as i32 - 15, self.body.position.y as i32 - 10, 30, 20, Color::LIGHTGRAY);
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

struct World {
    jetman: Jetman,
    items: Vec<Item>,
    gravity: Vector2,
}

impl World {
    fn new() -> Self {
        World {
            jetman: Jetman::new(),
            items: vec![Item::new(100.0, 200.0)],
            gravity: Vector2::new(0.0, 0.01),
        }
    }

    fn update(&mut self, is_space_down: bool, is_left_down: bool, is_right_down: bool) {
        if is_space_down {
            self.jetman.apply_thrust();
        }
        if is_left_down {
            self.jetman.turn_left();
        }
        if is_right_down {
            self.jetman.turn_right();
        }

        self.jetman.apply_force(self.gravity);
        self.jetman.update();
        
        for (id, item) in self.items.iter_mut().enumerate() {
            let diff = item.position() - self.jetman.position();
            let distance = diff.length();
            if distance < 20.0 {
                self.jetman.linked_item = Some(ItemId(id));
            }

            item.apply_force(self.gravity);
            item.update();
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        for item in &self.items {
            item.draw(d);
        }
        self.jetman.draw(d);
        if let Some(item_id) = self.jetman.linked_item {
            let item = &self.items[item_id.0 as usize];
            d.draw_line_v(self.jetman.position(), item.position(), Color::GREEN);
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
        let is_space_down = rl.is_key_down(KeyboardKey::KEY_SPACE);
        let is_left_down = rl.is_key_down(KeyboardKey::KEY_LEFT);
        let is_right_down = rl.is_key_down(KeyboardKey::KEY_RIGHT);
        let mut d = rl.begin_drawing(&thread);
        world.update(is_space_down, is_left_down, is_right_down);
        world.draw(&mut d);
    }
}
