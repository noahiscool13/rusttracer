use crate::util::color::Color;
use crate::datastructure::intersection::Intersection;
use crate::scene::Scene;

pub mod shaders;
pub mod testshader;

// TODO: recursive shading
pub trait Shader<'s> {
    fn new(scene: &'s Scene) -> Self;

    fn shade(&self, intersection: &Intersection) -> Color;
}