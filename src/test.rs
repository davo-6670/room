use crate::hit::World;
use crate::line::Line;
use crate::vec2::Vec2;

pub fn test_world1() -> World {
    let world: World = vec![
        Box::new(Line::new(Vec2::new(-1.0, 2.0), Vec2::new(1.0, 2.0))),
        Box::new(Line::new(Vec2::new(2.0, 1.0), Vec2::new(2.0, -1.0))),
        Box::new(Line::new(Vec2::new(-1.0, -2.0), Vec2::new(1.0, -2.0))),
        Box::new(Line::new(Vec2::new(-2.0, -1.0), Vec2::new(-2.0, 1.0))),
    ];

    world
}
