use crate::datastructure::DataStructure;
use crate::util::ray::Ray;
use crate::util::vector::Vector;

pub mod mcshader;
pub mod mtlshader;
pub mod shaders;
pub mod vmcshader;

/// A shader in the rusttracer codebase means a piece of code that takes a ray,
/// and asks the `datastructure` where it lands. Based on the `Intersection` struct
/// it gets back, it can give a color to a pixel. A shader can query the `datastructure`
/// multiple times to achieve such things as reflection, refraction, and other effects.
pub trait Shader: Sync + Send {
    fn shade<'s>(&self, ray: &Ray, datastructure: &'s (dyn DataStructure + 's)) -> Vector;
}
