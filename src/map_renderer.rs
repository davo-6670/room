use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use crate::line::Line;
use crate::ray::Ray;

pub struct MapRenderer {
    canvas: WindowCanvas,
    scale: f32,
}

impl MapRenderer {
    pub fn new(window: Window, scale: f32) -> Result<MapRenderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(MapRenderer { canvas, scale })
    }

    pub fn draw_line(&mut self, line: &Line) {
        let (start, end) = line.get_points();

        let (x1, y1) = start.xy();
        let (x2, y2) = end.xy();

        let (x1, y1) = (x1 * self.scale, y1 * self.scale);
        let (x2, y2) = (x2 * self.scale, y2 * self.scale);

        self.canvas
            .draw_line((x1 as i32, y1 as i32), (x2 as i32, y2 as i32))
            .unwrap();
    }

    // something ate a lot of memory so I'm not using it + is slow
    pub fn draw_ray(&mut self, ray: &Ray) {
        let (x1, y1) = ray.origin().xy();
        let (x2, y2) = ray.at(10000.0).xy();

        let (x1, y1) = (x1 * self.scale, y1 * self.scale);
        let (x2, y2) = (x2 * self.scale, y2 * self.scale);

        self.canvas
            .draw_line((x1 as i32, y1 as i32), (x2 as i32, y2 as i32))
            .unwrap();
    }

    pub fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
