use macroquad::prelude::*;

fn vector_from_angle(angle: f32) -> Vec2 {
    Vec2::new(angle.cos(), angle.sin())
}

/// A physics body.
#[derive(Clone, Copy)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
}

impl Body {
    /// Create a new body.
    pub fn new(position: Vec2, mass: f32) -> Self {
        Body {
            position,
            velocity: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
            mass,
        }
    }

    /// Apply a force to the body.
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }

    /// Clear all forces acting on the body.
    pub fn clear_forces(&mut self) {
        self.velocity *= 0.0;
        self.acceleration *= 0.0;
    }

    /// Update the body's position based on its velocity and acceleration.
    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
        self.acceleration = Vec2::ZERO;
    }
}

pub trait Bodied {
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
pub struct ItemId(pub usize);

pub struct Jetman {
    pub body: Body,
    pub heading: f32,
    pub link_distance: f32,
    pub linked_item: Option<ItemId>,
    pub thrusting: i32,
}

impl Jetman {
    pub fn new() -> Self {
        Jetman {
            body: Body::new(Vec2::new(200.0, 200.0), 1.0),
            heading: 0.0,
            link_distance: 50.0,
            linked_item: None,
            thrusting: 0,
        }
    }

    pub fn apply_thrust(&mut self) {
        let thrust = vector_from_angle(self.heading) * 0.1;
        self.body.apply_force(thrust);
        self.thrusting = 2;
    }

    pub fn turn_left(&mut self) {
        self.heading -= 0.1;
    }

    pub fn turn_right(&mut self) {
        self.heading += 0.1;
    }

    pub fn update(&mut self, dt: f32) {
        self.body.update(dt);
        self.thrusting -= 1;
    }

    pub fn draw(&self) {
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

impl Default for Jetman {
    fn default() -> Self {
        Jetman::new()
    }
}

pub struct Item {
    pub body: Body,
}

impl Item {
    pub fn new(x: f32, y: f32) -> Self {
        Item {
            body: Body::new(Vec2::new(x, y), 1.0),
        }
    }

    pub fn draw(&self) {
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
pub struct Teleporter {
    pub position: Vec2,
}

impl Teleporter {
    pub fn new(position: Vec2) -> Self {
        Teleporter { position }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 10.0, YELLOW);
    }
}
