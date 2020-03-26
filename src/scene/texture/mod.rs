use image::{DynamicImage, ImageError, GenericImageView, RgbImage};
use std::path::Path;

mod textureatlas;

pub use textureatlas::{TextureAtlas, TextureAtlasBuilder};
use std::fmt::{Debug, Formatter};
use std::fmt;
use crate::util::vector::Vector;
use crate::scene::texturecoordinate::TextureCoordinate;

#[derive(Debug)]
pub enum TextureError {
    ImageError(ImageError),
    FileName
}

pub struct Texture {
    image: RgbImage
}

impl Debug for Texture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Texture")
    }
}

impl Texture {
    pub fn new(filename: impl AsRef<Path>) -> Result<Self, TextureError> {
        Ok(Self {
            image: image::open(filename).map_err(TextureError::ImageError)?.to_rgb(),
        })
    }

    pub fn at(&self, coord: TextureCoordinate) -> Vector {
        let rgb = self.image.get_pixel(coord.u as u32, coord.v as u32);

        Vector::new(rgb.0[0] as f64 / 255., rgb.0[1] as f64 / 255., rgb.0[2] as f64 / 255.)
    }
}