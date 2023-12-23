mod hit;
mod line;
mod map;
mod map_renderer;
mod ray;
mod test;
mod vec2;

use hit::{Hit, World};
use line::Line;
use map::load_map;
use map_renderer::MapRenderer;
use ray::Ray;
use vec2::Vec2;

use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
extern crate sdl2;

fn main() {
    let world = load_map("./map.png".to_string());
    println!("objects in world: {}", world.len());
    let ray = Ray::new(Vec2::new(1.0, 4.0), Vec2::new(20.0, 20.0));

    /*
    let world: World = vec![
        Box::new(Line::new(Vec2::new(10.0, 40.0), Vec2::new(30.0, 40.0))),
        Box::new(Line::new(Vec2::new(40.0, 30.0), Vec2::new(40.0, 10.0))),
        Box::new(Line::new(Vec2::new(10.0, 0.0), Vec2::new(30.0, 0.0))),
        Box::new(Line::new(Vec2::new(0.0, 10.0), Vec2::new(0.0, 30.0))),
    ];
    */

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
        map_renderer.draw_ray(&ray);
        map_renderer.present();

        let hit_rec = world.hit(&ray, 0.0, 1000.0);
        if let Some(hit_rec) = hit_rec {
            println!("{}", hit_rec.p);
        }

        sleep(Duration::from_secs(1));
    }
}
