use crate::algorithm::{MapGenerator, TileType};
use crate::error::Error;

use image::{Rgb, RgbImage};
use imageproc::definitions::HasBlack;
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;
use log::info;

pub struct ImageGenerator {
    width: u32,
    height: u32,
    title_size: u32,
    grid: bool,
}

impl ImageGenerator {
    pub fn new(width: u32, height: u32, title_size: u32) -> Self {
        Self {
            width,
            height,
            title_size,
            grid: false,
        }
    }

    pub fn set_grid(&mut self, value: bool) {
        self.grid = value;
    }

    pub fn get_title_width(&self) -> u32 {
        self.width / self.title_size
    }

    pub fn get_title_height(&self) -> u32 {
        self.height / self.title_size
    }

    pub fn create<G: MapGenerator>(&self, mut generator: G) -> Result<RgbImage, Error> {
        let title_width = self.get_title_width();
        let title_height = self.get_title_height();

        info!("map creation");
        generator.generate()?;
        info!("map done");

        info!("map -> image buffer");
        let mut buffer = RgbImage::new(self.width, self.height);
        for i in 0..title_width {
            for j in 0..title_height {
                let rect = Rect::at((i * self.title_size) as i32, (j * self.title_size) as i32)
                    .of_size(self.title_size, self.title_size);
                let title_type = generator.get_title(i, j);
                let color = match title_type {
                    TileType::Floor => Rgb::from([255, 0, 0]),
                    TileType::Wall => Rgb::from([0, 255, 0]),
                    TileType::Door => Rgb::from([238, 130, 238]),
                };
                draw_filled_rect_mut(&mut buffer, rect, color);
            }
        }
        if self.grid {
            info!("add grid");
            for i in 0..title_width {
                draw_line_segment_mut(
                    &mut buffer,
                    ((i * self.title_size) as f32, 0.0),
                    ((i * self.title_size) as f32, self.height as f32 - 1.0),
                    Rgb::black(),
                );
            }
            for i in 0..title_height {
                draw_line_segment_mut(
                    &mut buffer,
                    (0.0, (i * self.title_size) as f32),
                    (self.width as f32 - 1.0, (i * self.title_size) as f32),
                    Rgb::black(),
                );
            }
        }
        info!("image buffer done");
        Ok(buffer)
    }
}
