use crate::vec2::Vec2;

pub struct Ray {
    direction: Vec2,
    origin: Vec2,
}

impl Ray {
    pub fn new(direction: Vec2, origin: Vec2) -> Ray {
        Ray { direction, origin }
    }

    pub fn at(&self, t: f32) -> Vec2 {
        self.origin + t * self.direction
    }

    pub fn direction(&self) -> Vec2 {
        self.direction
    }

    pub fn origin(&self) -> Vec2 {
        self.origin
    }
}
