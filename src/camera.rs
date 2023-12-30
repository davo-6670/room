use crate::ray::Ray;
use crate::vec2::Vec2;

pub struct Camera {
    origin: Vec2,
    rotation:
}

impl Camera {
    pub fn new(origin: Vec2, fov: f32) -> Camera {
        let theta = std::f32::consts::PI/180.0 * fov;
        let viewport_width = 2.0 * (theta / 2.0).tan();
        Camera {
            origin,
            rotation,
        }
    }

    pub fn get_ray(&self, n: f32) -> Ray {
        Ray::new(
            Vec2::new(self.image_width / self.rows * n, self.y_dist - n),
            self.pos,
        )
    }
}
