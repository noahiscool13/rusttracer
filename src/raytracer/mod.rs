use crate::datastructure::DataStructure;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::vector::Vector;
use serde::export::fmt::Debug;

pub mod basic;
pub mod jmstracer;
pub mod mstracer;

/// A raytracer is a struct that takes an x and y coordinate on the screen,
/// and generates a ray associated with that coordinate. Then this ray can be passed
/// to a shader to get a color associated with this x-y coordinate.
pub trait RayTracer: Send + Sync + Debug {
    fn raytrace<'r>(
        &self,
        x: usize,
        y: usize,
        datastructure: &'r dyn DataStructure,
        shader: &'r dyn Shader,
        camera: &Camera,
    ) -> Vector;
}
