mod cube;
mod hit;
mod line;
mod map;
mod ray;
mod test;
mod vec2;

use hit::{Hit, World};
use line::Line;
use map::load_map;
use ray::Ray;
use test::test_world1;
use vec2::Vec2;

use rayon::prelude::*;
use std::time::Instant;

use crate::hit::HitRecord;

fn main() {
    let world = test_world1();
    /*
    let mut world = World::new();
    world.push(Box::new(Line::new(
        Vec2::new(-1.0, 2.0),
        Vec2::new(1.0, 2.0),
    )));
    world.push(Box::new(Line::new(
        Vec2::new(-1.0, 1.0),
        Vec2::new(1.0, 1.0),
    )));
    */
    let new_ray = Ray::new(Vec2::new(0.0, 1.0), Vec2::new(0.0, 0.0));

    let now = Instant::now();

    let scanline: Vec<Option<HitRecord>> = (0..1e5 as i32)
        .into_par_iter()
        .map(|_| {
            let x = world.hit(&new_ray, 0.0, 1000.0);

            x
        })
        .collect();
    let w = now.elapsed().as_millis();
    println!("took {} ms to calculate {} lines", w, 1e5);
    let _ = load_map("./map.png".to_string());
}
