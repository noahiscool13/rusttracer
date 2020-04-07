use crate::datastructure::intersection::Intersection;
use crate::util::ray::Ray;

pub mod basic;
pub mod intersection;
pub mod precalculated;

pub trait DataStructure<'d>: Sync {
    /// Returns the closest intersection
    fn intersects<'a>(&'a self, ray: &'a Ray) -> Option<Intersection<'a>>;
}
