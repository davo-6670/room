use crate::line::Line;
use crate::material::Color;
use crate::{hit::World, vec2::Vec2};

use image::GenericImageView;

pub fn load_map(path: String) -> World {
    let map_img = image::open(path).unwrap().into_rgb8();
    let (image_width, image_height) = map_img.dimensions();
    let red = Color::new(255, 0, 0);

    let mut world = World::new();

    for y in 0..image_height {
        for x in 0..image_width {
            if map_img.get_pixel(x, y) == &image::Rgb([0_u8, 0_u8, 0_u8]) {
                // checking above
                if map_img.in_bounds(x, (y as i32 - 1_i32) as u32) {
                    if !(map_img.get_pixel(x, y - 1) == &image::Rgb([0_u8, 0_u8, 0_u8])) {
                        world.push(Box::new(Line::new(
                            Vec2::new(x as f32, y as f32),
                            Vec2::new(x as f32 + 1., y as f32),
                            red,
                        )));
                    }
                } else {
                    world.push(Box::new(Line::new(
                        Vec2::new(x as f32, y as f32),
                        Vec2::new(x as f32 + 1., y as f32),
                        red,
                    )));
                }
                // cheking right side
                if map_img.in_bounds(x + 1, y) {
                    if !(map_img.get_pixel(x + 1, y) == &image::Rgb([0_u8, 0_u8, 0_u8])) {
                        world.push(Box::new(Line::new(
                            Vec2::new(x as f32 + 1., y as f32),
                            Vec2::new(x as f32 + 1., y as f32 + 1.),
                            red,
                        )));
                    }
                } else {
                    world.push(Box::new(Line::new(
                        Vec2::new(x as f32 + 1., y as f32),
                        Vec2::new(x as f32 + 1., y as f32 + 1.),
                        red,
                    )));
                }
                // checking under
                if map_img.in_bounds(x, y + 1) {
                    if !(map_img.get_pixel(x, y + 1) == &image::Rgb([0_u8, 0_u8, 0_u8])) {
                        world.push(Box::new(Line::new(
                            Vec2::new(x as f32 + 1., y as f32 + 1.),
                            Vec2::new(x as f32, y as f32 + 1.),
                            red,
                        )));
                    }
                } else {
                    world.push(Box::new(Line::new(
                        Vec2::new(x as f32 + 1., y as f32 + 1.),
                        Vec2::new(x as f32, y as f32 + 1.),
                        red,
                    )));
                }
                // checking left side
                if map_img.in_bounds((x as i32 - 1_i32) as u32, y) {
                    if !(map_img.get_pixel(x - 1, y) == &image::Rgb([0_u8, 0_u8, 0_u8])) {
                        world.push(Box::new(Line::new(
                            Vec2::new(x as f32, y as f32 + 1.),
                            Vec2::new(x as f32, y as f32),
                            red,
                        )));
                    }
                } else {
                    world.push(Box::new(Line::new(
                        Vec2::new(x as f32, y as f32 + 1.),
                        Vec2::new(x as f32, y as f32),
                        red,
                    )));
                }

                /* world world gen code
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
                */
            }
        }
    }

    world
}
