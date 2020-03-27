use crate::error::RustTracerResult;

///! This module defines a render setup. It basically sets up a predefined pipeline of postprocessors, cameras, shaders and renderers.

pub mod monte_carlo;
pub mod monte_carlo_preview;

pub trait Setup {
    fn run(&self) -> RustTracerResult<()>;
}