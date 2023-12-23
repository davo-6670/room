use crate::hit::{Hit, HitRecord};
use crate::map_renderer::MapRenderer;
use crate::ray::Ray;
use crate::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Line {
    start: Vec2,
    end: Vec2,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2) -> Line {
        Line { start, end }
    }

    pub fn get_points(self) -> (Vec2, Vec2) {
        (self.start, self.end)
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

        if t > t_min && t < t_max && (0.0..=1.0).contains(&u) {
            Some(HitRecord { p: r.at(t), t })
        } else {
            None
        }
    }

    fn draw(&self, renderer: &mut MapRenderer) {
        renderer.draw_line(self);
    }
}
