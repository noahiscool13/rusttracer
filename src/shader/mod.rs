use crate::util::vector::Vector;
use crate::datastructure::DataStructure;
use crate::util::ray::Ray;

pub mod shaders;
pub mod testshader;
pub mod mtlshader;
pub mod mcshader;
pub mod vmcshader;

// TODO: recursive shading
pub trait Shader<'s, DS: DataStructure<'s>> {
    fn shade(&self, ray: Ray, datastructure : &DS) -> Vector;
}