use crate::previewer::PreviewerError;
use crate::scene::error::SceneError;

#[derive(Debug)]
pub enum RustTracerError {
    PreviewError(PreviewerError),
    TobjError(tobj::LoadError),
    SceneError(SceneError)
}

pub type RustTracerResult<T> = Result<T, RustTracerError>;
