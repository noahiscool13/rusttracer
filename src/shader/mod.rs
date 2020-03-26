use crate::util::color::Color;
use crate::datastructure::intersection::Intersection;
use crate::scene::Scene;
use crate::datastructure::DataStructure;
use crate::util::vector::Vector;

pub mod shaders;
pub mod testshader;
pub mod mtlshader;
pub mod mcshader;

// TODO: recursive shading
pub trait Shader<'s, DS: DataStructure<'s>> {
    fn new(scene: &'s Scene) -> Self;

    fn shade(&self, intersection: &Intersection, datastructure : &DS) -> Vector;
}