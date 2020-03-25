use crate::util::ray::Ray;
use crate::util::vector::Vector;
use crate::scene::Face;

#[derive(Debug)]
pub struct Intersection<'i> {
    pub ray: &'i Ray,
    pub uv: (f64, f64),
    pub t: f64,
    pub face: Face
}

impl<'i> Intersection<'i> {
    pub fn hit_pos(&self) -> Vector {
        self.ray.origin + self.ray.direction*self.t
    }
}