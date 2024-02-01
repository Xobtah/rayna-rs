use std::path::Path;

use raylib::color::Color;

#[derive(Debug, Clone)]
pub struct Texture {
    pub pixels: Vec<Color>,
    pub width: u32,
    pub height: u32,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            pixels: vec![Color::VIOLET, Color::BLACK, Color::BLACK, Color::VIOLET],
            width: 2,
            height: 2,
        }
    }
}

impl Texture {
    pub fn from_png(path: &Path) -> Result<Self, ()> {
        let image = image::open(path).map_err(|_| ())?.to_rgba8();
        let (width, height) = image.dimensions();
        let pixels = image
            .into_raw()
            .chunks(4)
            .map(|chunk| Color::new(chunk[0], chunk[1], chunk[2], chunk[3]))
            .collect::<Vec<Color>>();
        Ok(Self {
            pixels,
            width,
            height,
        })
    }

    pub fn get_line(&self, x: u8, line_target_height: u32) -> Vec<Color> {
        if line_target_height == 0 {
            return Vec::new();
        }
        let x = (x as f32 / 255.0 * self.width as f32) as u32;
        let line = (0..self.height)
            .map(|i| self.pixels[(i * self.width + x) as usize])
            .collect::<Vec<Color>>();
        (0..line_target_height)
            .map(|i| line[(i * self.height / line_target_height) as usize])
            .collect()
    }
}
