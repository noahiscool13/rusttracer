use crate::scene::texture::TextureError;

#[derive(Debug)]
pub enum SceneError {
    TextureError(TextureError),
}

impl From<TextureError> for SceneError {
    fn from(t: TextureError) -> Self {
        SceneError::TextureError(t)
    }
}
