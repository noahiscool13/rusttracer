use std::collections::HashMap;
use std::path::Path;
use crate::scene::texture::{Texture, TextureError};

pub struct TextureAtlas {
    atlas: HashMap<String, Texture>
}

impl TextureAtlas {
    pub fn new() -> Self {
        Self {
            atlas: HashMap::new()
        }
    }

    pub fn add_texture_file(&mut self, filename: impl AsRef<Path>) -> Result<(), TextureError> {
        Ok(self.add_texture(filename.as_ref()
                                .to_str()
                                .ok_or(TextureError::FileName)?
                                .into(),
                            Texture::new(filename)?
        ))
    }

    pub fn add_texture(&mut self, name: String, texture: Texture) {
        self.atlas.insert(name, texture);
    }

    pub fn get_texture(&self, name: &String) -> Option<&Texture> {
        self.atlas.get(name)
    }

    pub fn get_or_add_texture(&mut self, filename: impl AsRef<Path>) -> Result<&Texture, TextureError>  {
        let strname = filename.as_ref()
            .to_str()
            .ok_or(TextureError::FileName)?
            .into();

        Ok(self.atlas.entry(strname).or_insert(Texture::new(filename)?))
    }
}