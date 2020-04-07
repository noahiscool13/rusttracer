use crate::datastructure::DataStructure;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;

pub mod basic;
pub mod crossbeamjmstrace;
pub mod jmstrace;
pub mod mstrace;
pub mod rayon;

pub trait RayTracer<'r> {
    fn raytrace(
        &self,
        datastructure: &'r dyn DataStructure<'r>,
        shader: &'r dyn Shader<'r>,
        camera: &Camera,
    ) -> OutputBuffer;
}
