use crate::scene::texture::TextureError;
use crate::scene::light::LightError;

#[derive(Debug)]
pub enum SceneError {
    TextureError(TextureError),
    LightError(LightError),
}

impl From<TextureError> for SceneError {
    fn from(t: TextureError) -> Self {
        SceneError::TextureError(t)
    }
}
impl From<LightError> for SceneError {
    fn from(l: LightError) -> Self {
        SceneError::LightError(l)
    }
}
