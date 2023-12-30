mod hit;
mod line;
mod map;
mod map_renderer;
mod material;
mod ray;
mod test;
mod vec2;

use hit::{Hit, HitRecord, World};
use line::Line;
use map::load_map;
use map_renderer::MapRenderer;
use ray::Ray;
use vec2::Vec2;

use std::thread::sleep;
use std::time::Duration;

use rayon::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
extern crate sdl2;

const IMAGE_WIDTH: f32 = 100.0;

fn main() {
    let world = load_map("./map.png".to_string());

    // all of the sdl2 init stuff
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Map", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut map_renderer = MapRenderer::new(window, 15.0).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => (),
            }
        }
        map_renderer.clear(Color::RGB(255, 255, 255));
        map_renderer.set_draw_color(Color::RGB(0, 0, 0));
        world.draw(&mut map_renderer);

        /*
        for n in 0..IMAGE_WIDTH as u32 {
            let ray = Ray::new(Vec2::new(5.0, 2.0), Vec2::new(0.0, 0.0));
            let hit_rec = world.hit(&ray, 0.0, 10000.0);
        }
        */
        let scanline: Vec<u8> = (0..IMAGE_WIDTH as u32)
            .into_par_iter()
            .map(|i| {
                let ray = Ray::new(Vec2::new(5.0, 2.0), Vec2::new(5.0, 10.0));
                let hit_rec = world.hit(&ray, 0.0, 10000.0);
                if let Some(_) = hit_rec {
                    return 1;
                }
                return 0;
            })
            .collect();
        map_renderer.present();
    }
}
