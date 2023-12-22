use crate::line::Line;
use crate::vec2::Vec2;

pub struct Cube {
    position: Vec2,
    walls: [Line; 4],
}

impl Default for Cube {
    fn default() -> Self {
        Cube {
            position: Vec2::new(0.0, 0.0),
            walls: [
                Line::new(Vec2::new(-0.5, -0.5), Vec2::new(-0.5, 0.5)),
                Line::new(Vec2::new(-0.5, 0.5), Vec2::new(0.5, 0.5)),
                Line::new(Vec2::new(0.5, 0.5), Vec2::new(0.5, -0.5)),
                Line::new(Vec2::new(0.5, -0.5), Vec2::new(-0.5, -0.5)),
            ],
        }
    }
}
