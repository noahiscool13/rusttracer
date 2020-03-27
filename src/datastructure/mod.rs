use crate::util::ray::Ray;
use crate::datastructure::intersection::Intersection;
use crate::scene::scene::Scene;

pub mod basic;
pub mod intersection;

pub trait DataStructure<'d> {
    /// Generates a datastructure
    fn new(obj: &'d Scene<'d>) -> Self;

    /// Returns the closest intersection
    fn intersects<'a>(&'a self, ray: &'a Ray) -> Option<Intersection<'a>>;
}
