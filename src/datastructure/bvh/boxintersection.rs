use crate::datastructure::bvh::boundingbox::BoundingBox;
use crate::util::ray::Ray;

#[derive(Debug)]
pub struct BoxIntersection<'i> {
    pub ray: &'i Ray,
    pub t: f64,
    pub boundingbox: &'i BoundingBox,
}
