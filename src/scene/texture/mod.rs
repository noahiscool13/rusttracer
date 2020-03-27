use image::{ImageError, GenericImageView, RgbImage};
use std::path::Path;

mod textureatlas;

pub use textureatlas::{TextureAtlas, TextureAtlasBuilder};
use std::fmt::{Debug, Formatter};
use std::fmt;
use crate::util::vector::Vector;
use crate::scene::texturecoordinate::TextureCoordinate;
use log::error;

#[derive(Debug)]
pub enum TextureError {
    ImageError(ImageError),
    FileName
}

pub struct Texture {
    image: RgbImage,
    size: (usize, usize),
}

impl Debug for Texture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Texture")
    }
}

impl Texture {
    pub fn new(filename: impl AsRef<Path>) -> Result<Self, TextureError> {
        let image = image::open(filename).map_err(TextureError::ImageError)?;
        let dimensions = image.dimensions();

        Ok(Self {
            image: image.to_rgb(),
            size: (dimensions.0 as usize, dimensions.1 as usize)
        })
    }

    pub fn at(&self, coord: TextureCoordinate) -> Vector {

//        error!("u: {}, v: {}", coord.u, coord.v);

        let x = (coord.u * self.size.0 as f64) as u32;
        let y = (self.size.1 - (coord.v * self.size.1 as f64) as usize) as u32;

        let rgb = self.image.get_pixel(x, y);

        Vector::new(rgb.0[0] as f64 / 255., rgb.0[1] as f64 / 255., rgb.0[2] as f64 / 255.)
    }
}