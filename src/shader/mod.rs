use crate::util::color::Color;
use crate::datastructure::intersection::Intersection;
use crate::util::vector::Vector;
use crate::scene::scene::Scene;
use crate::scene::triangle::Triangle;

pub mod testshader;
pub mod mtlshader;
pub mod shaders;

// TODO: recursive shading
pub trait Shader<'s> {
    fn new(scene: &'s Scene<'s>) -> Self;
    fn shade(&self, intersection: &Intersection) -> Color;
}