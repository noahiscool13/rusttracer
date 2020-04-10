use crate::datastructure::DataStructure;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;
use crate::util::ray::Ray;
use crate::util::vector::Vector;

pub mod basic;
pub mod crossbeamjmstrace;
pub mod jmstrace;
pub mod mstrace;
pub mod rayon;


pub trait RayTracer {
    fn raytrace<'r>(
        &self,
        datastructure: &'r (dyn DataStructure + 'r),
        shader: &'r (dyn Shader + 'r),
        camera: &Camera,
    ) -> OutputBuffer;

}
