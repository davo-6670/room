use crate::hit::World;

use image::GenericImageView;

pub fn load_map(path: String) -> World {
    let map_img = image::open(path).unwrap();
    let (image_width, image_height) = map_img.dimensions();

    println!(
        "image width: {}, image height {}",
        image_width, image_height
    );

    for y in 0..image_height {
        for x in 0..image_width {
            if map_img.get_pixel(x, y) == image::Rgba([255_u8, 255_u8, 255_u8, 255_u8]) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!();
    }

    World::default()
}
