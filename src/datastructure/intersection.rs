use crate::scene::triangle::Triangle;
use crate::util::ray::Ray;
use crate::util::vector::Vector;
use std::f64::EPSILON;

#[derive(Debug)]
/// Represents the intersection point between a ray and a triangle.
pub struct Intersection<'i> {
    /// A reference to the original ray that was used to get this intersection.
    pub ray: &'i Ray,
    /// the uv (barycentric) coordinates of the hitpoint on the triangle.
    pub uv: (f64, f64),
    /// The distance from the ray origin to the hitpoint on the triangle.
    pub t: f64,
    /// A reference to the triangle that was hit by the ray.
    pub triangle: &'i Triangle<'i>,
}

impl<'i> Intersection<'i> {
    /// Returns a point in 3d space where the hit occured.
    /// TODO: 3d point in worldspace right?
    pub fn hit_pos(&self) -> Vector {
        self.ray.origin + self.ray.direction * (self.t - EPSILON)
    }
}
