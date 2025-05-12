use macroquad::prelude::*;

use crate::physics::Body;

pub enum TerrainShape {
    Rectangle(Rect),
    Line(Vec2, Vec2),
    Circle(Vec2, f32),
}

pub struct Terrain {
    shape: TerrainShape,
}

impl Terrain {
    pub fn rectangle(x: f32, y: f32, w: f32, h: f32) -> Self {
        Terrain {
            shape: TerrainShape::Rectangle(Rect::new(x, y, w, h)),
        }
    }

    pub fn line(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Terrain {
            shape: TerrainShape::Line(Vec2::new(x1, y1), Vec2::new(x2, y2)),
        }
    }

    pub fn draw(&self) {
        match self.shape {
            TerrainShape::Rectangle(rect) => {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, DARKGREEN);
            }
            TerrainShape::Line(a, b) => {
                draw_line(a.x, a.y, b.x, b.y, 4.0, DARKGREEN);
            }
            TerrainShape::Circle(c, r) => {
                draw_circle(c.x, c.y, r, DARKGREEN);
            }
        }
    }
}

pub fn check_collision(body: &mut Body, terrain: &Terrain) {
    match terrain.shape {
        TerrainShape::Rectangle(rect) => {
            let pos = body.position;
            if pos.x > rect.x
                && pos.x < rect.x + rect.w
                && pos.y > rect.y
                && pos.y < rect.y + rect.h
            {
                body.position.y = rect.y - 1.0;
                body.velocity.y = -body.velocity.y * 0.5;
            }
        }
        TerrainShape::Line(p1, p2) => {
            let pos = body.position;
            let line = p2 - p1;
            let to_pos = pos - p1;
            let len_sq = line.length_squared();
            if len_sq == 0.0 {
                return;
            }

            let t = (to_pos.dot(line) / len_sq).clamp(0.0, 1.0);
            let closest = p1 + line * t;
            let dist = (pos - closest).length();

            if dist < 10.0 {
                let normal = (pos - closest).normalize();
                body.position = closest + normal * 10.0;
                body.velocity -= 2.0 * body.velocity.dot(normal) * normal;
                body.velocity *= 0.5;
            }
        }
        TerrainShape::Circle(center, radius) => {
            let pos = body.position;
            let delta = pos - center;
            let dist = delta.length();
            let min_dist = radius + 10.0;

            if dist < min_dist {
                let normal = delta.normalize();
                body.position = center + normal * min_dist;
                body.velocity -= 2.0 * body.velocity.dot(normal) * normal;
                body.velocity *= 0.5;
            }
        }
    }
}
