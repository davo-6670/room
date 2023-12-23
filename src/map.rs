use crate::line::Line;
use crate::{hit::World, vec2::Vec2};

use image::GenericImageView;

pub fn load_map(path: String) -> World {
    let map_img = image::open(path).unwrap().into_rgb8();
    let (image_width, image_height) = map_img.dimensions();

    println!(
        "image width: {}, image height {}",
        image_width, image_height
    );

    let mut world = World::new();

    let wdwa = map_img.get_pixel(2, 2);
    dbg!(wdwa);
    let wdwa = map_img.get_pixel(0, 0);
    dbg!(wdwa);

    for y in 0..image_height {
        for x in 0..image_width {
            if map_img.get_pixel(x, y) == &image::Rgb([0_u8, 0_u8, 0_u8]) {
                print!("#");
                world.push(Box::new(Line::new(
                    Vec2::new(x as f32, y as f32),
                    Vec2::new(x as f32 + 1., y as f32),
                )));
                world.push(Box::new(Line::new(
                    Vec2::new(x as f32 + 1., y as f32),
                    Vec2::new(x as f32 + 1., y as f32 + 1.),
                )));
                world.push(Box::new(Line::new(
                    Vec2::new(x as f32 + 1., y as f32 + 1.),
                    Vec2::new(x as f32, y as f32 + 1.),
                )));
                world.push(Box::new(Line::new(
                    Vec2::new(x as f32, y as f32 + 1.),
                    Vec2::new(x as f32, y as f32),
                )));
            } else {
                print!(" ");
            }
        }
        println!();
    }

    world
}
