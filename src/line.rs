use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;
use crate::vec2::Vec2;

pub struct Line {
    start: Vec2,
    end: Vec2,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2) -> Line {
        Line { start, end }
    }
}

impl Hit for Line {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let ray_to_line_start = self.start - r.origin();
        let num_t = ray_to_line_start.cross(self.start - self.end);
        let num_u = r.direction().cross(ray_to_line_start);

        // check for parrale lines
        let denominator = r.direction().cross(self.start - self.end);
        if denominator < 1e-9 {
            return None;
        }

        let t = num_t / denominator;
        let u = num_u / denominator;

        if t > t_min && t < t_max && 0.0 <= u && u <= 1.0 {
            return Some(HitRecord { p: r.at(t), t: t });
        } else {
            return None;
        }
    }
}
