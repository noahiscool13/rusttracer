use crate::util::color::Color;
use crate::datastructure::intersection::Intersection;
use crate::util::vector::Vector;
use crate::scene::scene::Scene;
use crate::scene::triangle::Triangle;
use crate::datastructure::DataStructure;

pub mod shaders;
pub mod testshader;
pub mod mtlshader;
pub mod mcshader;

// TODO: recursive shading
pub trait Shader<'s, DS: DataStructure<'s>> {

    fn new(scene: &'s Scene<'s>) -> Self;
    fn shade(&self, intersection: &Intersection, datastructure : &DS) -> Vector;
}