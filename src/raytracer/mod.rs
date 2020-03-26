use crate::datastructure::DataStructure;
use crate::util::outputbuffer::OutputBuffer;
use crate::util::camera::Camera;
use crate::shader::Shader;

pub mod basic;
pub mod rayon;
pub mod mstrace;

pub trait RayTracer<'r, DS: DataStructure<'r>, S: Shader<'r, DS>> {
    fn new() -> Self;
    fn raytrace(&self, datastructure: &DS, shader: &S, camera: &Camera) -> OutputBuffer;
}