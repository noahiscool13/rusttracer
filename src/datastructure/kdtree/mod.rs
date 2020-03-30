use crate::scene::scene::Scene;
use crate::datastructure::DataStructure;
use crate::util::ray::Ray;
use crate::datastructure::intersection::Intersection;
use crate::datastructure::kdtree::boundingbox::BoundingBox;

mod boundingbox;


pub struct KDTreeDataStructure<'d> {
    data: &'d Scene<'d>,
}

impl<'d> KDTreeDataStructure<'d> {

}

impl<'d> DataStructure<'d> for KDTreeDataStructure<'d> {
    fn new(scene: &'d Scene<'d>) -> Self {
        Self { data: scene }
    }

    fn intersects<'a>(&'a self, ray: &'a Ray) -> Option<Intersection<'a>> {

        None
    }
}
