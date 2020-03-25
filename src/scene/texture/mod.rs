use image::{DynamicImage, ImageError};
use std::path::Path;

mod textureatlas;

pub use textureatlas::TextureAtlas;

#[derive(Debug)]
pub enum TextureError {
    ImageError(ImageError),
    FileName
}

pub struct Texture {
    image: DynamicImage
}

impl Texture {
    pub fn new(filename: impl AsRef<Path>) -> Result<Self, TextureError> {
        Ok(Self {
            image: image::open(filename).map_err(TextureError::ImageError)?,
        })
    }
}