use macroquad::prelude::*;

use crate::physics::Body;

/// Shape of a terrain element
pub enum TerrainShape {
    /// Rectangular terrain shape, axis-aligned
    Rectangle(Rect),
    /// Linear terrain shape
    Line(Vec2, Vec2),
    /// Circular terrain shape
    Circle(Vec2, f32),
    /// Polygonal terrain shape
    Polygon(Vec<Vec2>),
}

/// A terrain element. Jetman can collide with these.
pub struct Terrain {
    shape: TerrainShape,
}

impl Terrain {
    /// Create an axis-aligned rectangular terrain
    pub fn rectangle(x: f32, y: f32, w: f32, h: f32) -> Self {
        Terrain {
            shape: TerrainShape::Rectangle(Rect::new(x, y, w, h)),
        }
    }

    /// Create a linear terrain
    pub fn line(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Terrain {
            shape: TerrainShape::Line(Vec2::new(x1, y1), Vec2::new(x2, y2)),
        }
    }

    /// Create a circular terrain
    pub fn circle(x: f32, y: f32, r: f32) -> Self {
        Terrain {
            shape: TerrainShape::Circle(Vec2::new(x, y), r),
        }
    }

    pub fn polygon(segments: Vec<Vec2>) -> Self {
        Terrain {
            shape: TerrainShape::Polygon(segments),
        }
    }

    /// Draw the terrain element
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
            TerrainShape::Polygon(ref points) => {
                for i in 0..points.len() {
                    let a = points[i];
                    let b = points[(i + 1) % points.len()];
                    draw_line(a.x, a.y, b.x, b.y, 2.0, LIME);
                }
            }
        }
    }
}

/// Check for collisions between a body and a terrain
/// and alter the body's position and velocity on collision
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
        TerrainShape::Polygon(ref vertices) => {
            if point_in_polygon(body.position, vertices) {
                body.position.y -= 2.0; // crude correction
                body.velocity.y = -body.velocity.y * 0.5;
            }
        }
    }
}

fn point_in_polygon(point: Vec2, polygon: &[Vec2]) -> bool {
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let pi = polygon[i];
        let pj = polygon[j];
        if ((pi.y > point.y) != (pj.y > point.y))
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y + 0.00001) + pi.x)
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}
