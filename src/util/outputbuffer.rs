use crate::util::color::Color;
use bmp::{Image, px, Pixel};
use std::ops::{Deref, DerefMut};

pub struct OutputBuffer {
    buffer: Vec<Vec<Color>>,
}

impl Deref for OutputBuffer {
    type Target = Vec<Vec<Color>>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for OutputBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

impl OutputBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn presize(&mut self, width: usize, height: usize) {
        self.buffer.clear();
        self.buffer.reserve(height);
        for i in 0..height {
            self.buffer.push(Vec::new());
            self.buffer[i].resize_with(width, Default::default)
        }
    }

    pub fn with_size(width: usize, height: usize) -> Self{
        let mut res = Self::new();
        res.presize(width, height);
        res
    }

    pub fn from_buffer(buffer: Vec<Vec<Color>>) -> Self {
        Self { buffer }
    }

    pub fn to_bmp(&self) -> Image {
        let height = self.buffer.len();
        let width = if height > 0 {
            self.buffer[0].len()
        } else { 0 };

        let mut img = Image::new(width as u32, height as u32);

        for (x, y) in img.coordinates() {
            let color = &self.buffer[y as usize][x as usize];
            img.set_pixel(x, y, px!(color.r, color.g, color.b));
        }

        img
    }

    pub fn set_at(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[y][x] = color;
    }
}

impl Default for OutputBuffer {
    fn default() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }
}
