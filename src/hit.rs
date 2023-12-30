use crate::map_renderer::MapRenderer;
use crate::material::Color;
use crate::ray::Ray;
use crate::vec2::Vec2;

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }

    fn draw(&self, renderer: &mut MapRenderer) {
        for object in self {
            object.draw(renderer);
        }
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn draw(&self, renderer: &mut MapRenderer);
}

pub struct HitRecord {
    pub p: Vec2,
    pub t: f32,
    pub c: Color,
}
