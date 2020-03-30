use crate::datastructure::DataStructure;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;

pub mod basic;
pub mod jmstrace;
pub mod mstrace;
pub mod rayon;

pub trait RayTracer<'r, DS: DataStructure<'r>, S: Shader<'r, DS>> {
    fn raytrace(&self, datastructure: &DS, shader: &S, camera: &Camera) -> OutputBuffer;
}
