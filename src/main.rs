use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 400;
const SCREEN_HEIGHT: i32 = 400;

const TITLE: &str = "Jetman";

fn vector_from_angle(angle: f32) -> Vector2 {
    Vector2::new(angle.cos(), angle.sin())
}

#[derive(Clone, Copy)]
struct ItemId(usize);

struct Jetman {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    heading: f32,
    linked_item: Option<ItemId>,
}

impl Jetman {
    fn new() -> Self {
        Jetman {
            position: Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            heading: 0.0,
            linked_item: None,
        }
    }

    fn apply_force(&mut self, force: Vector2) {
        self.acceleration += force;
    }

    fn apply_thrust(&mut self) {
        let thrust = vector_from_angle(self.heading) * 0.1;
        self.apply_force(thrust);
    }

    fn turn_left(&mut self) {
        self.heading -= 0.1;
    }

    fn turn_right(&mut self) {
        self.heading += 0.1;
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.position, 10.0, Color::LIGHTBLUE);
        d.draw_circle_lines(self.position.x as i32, self.position.y as i32, 10.0, Color::BLUE);
        let tip = self.position + vector_from_angle(self.heading) * 20.0;
        d.draw_line_v(self.position, tip, Color::RED);
    }
}

struct Item {
    position: Vector2,
}

impl Item {
    fn new(x: f32, y: f32) -> Self {
        Item {
            position: Vector2::new(x, y),
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.position.x as i32 - 15, self.position.y as i32 - 10, 30, 20, Color::LIGHTGRAY);
    }
}

struct World {
    jetman: Jetman,
    items: Vec<Item>,
    gravity: f32,
}

impl World {
    fn new() -> Self {
        World {
            jetman: Jetman::new(),
            items: vec![Item::new(100.0, 200.0)],
            gravity: 0.01,
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
        self.jetman.apply_force(Vector2::new(0.0, self.gravity));
        self.jetman.update();
        for (id, item) in self.items.iter().enumerate() {
            let diff = item.position - self.jetman.position;
            let distance = diff.length();
            if distance < 20.0 {
                self.jetman.linked_item = Some(ItemId(id));
            }
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
            d.draw_line_v(self.jetman.position, item.position, Color::GREEN);
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
