use crate::scene::texture::{Texture, TextureError};
use image::DynamicImage;
use std::collections::HashMap;
use std::path::Path;
use std::pin::Pin;

pub struct TextureAtlasBuilder {
    atlas: HashMap<String, Texture>,
}

impl TextureAtlasBuilder {
    pub fn new() -> Self {
        Self {
            atlas: HashMap::new(),
        }
    }

    pub fn add_texture_file(
        &mut self,
        filename: impl AsRef<Path>,
        basepath: impl AsRef<Path>,
    ) -> Result<(), TextureError> {
        Ok(self.add_texture(
            filename
                .as_ref()
                .to_str()
                .ok_or(TextureError::FileName)?
                .into(),
            Texture::new(basepath.as_ref().join(filename))?,
        ))
    }

    pub fn add_texture(&mut self, name: String, texture: Texture) {
        self.atlas.insert(name, texture);
    }

    pub fn build<'t>(self) -> TextureAtlas<'t> {
        let mut atlas = HashMap::new();
        let atlassize = self.atlas.len();

        let mut textures = {
            let mut vec = Vec::with_capacity(atlassize);
            vec.resize_with(atlassize, || Texture {
                image: DynamicImage::new_rgb8(0, 0).to_rgb(),
                size: (0, 0),
            });
            Pin::from(vec.into_boxed_slice())
        };

        let mut names = Vec::with_capacity(atlassize);

        for (index, (key, value)) in self.atlas.into_iter().enumerate() {
            textures[index] = value;
            names.push(key);
        }

        for (index, name) in names.into_iter().enumerate() {
            // Safe because textures is pinned.
            let ptr: &'t Texture = unsafe { std::mem::transmute(&textures[index]) };

            atlas.insert(name, ptr);
        }

        TextureAtlas { atlas, textures }
    }
}

pub struct TextureAtlas<'t> {
    pub(self) atlas: HashMap<String, &'t Texture>,
    pub(self) textures: Pin<Box<[Texture]>>,
}

impl<'t> TextureAtlas<'t> {
    pub fn get_texture(&self, name: &String) -> Option<&'t Texture> {
        self.atlas.get(name).map(|&i| i)
    }
}
