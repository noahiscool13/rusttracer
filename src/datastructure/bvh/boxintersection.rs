use crate::util::ray::Ray;
use crate::datastructure::bvh::boundingbox::BoundingBox;

#[derive(Debug)]
pub struct BoxIntersection<'i> {
    pub ray: &'i Ray,
    pub t: f64,
    pub boundingbox: &'i BoundingBox,
}