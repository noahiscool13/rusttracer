use crate::datastructure::intersection::Intersection;
use crate::datastructure::bvh::boundingbox::BoundingBox;
use crate::datastructure::bvh::node::BVHNode;
use crate::datastructure::DataStructure;
use crate::scene::scene::Scene;
use crate::scene::triangle::Triangle;
use crate::util::ray::Ray;
use log::debug;
use std::collections::HashSet;

mod boundingbox;
mod node;

pub struct KDTreeDataStructure<'d> {
    root: BVHNode<'d>,
}

impl<'d> KDTreeDataStructure<'d> {}

impl<'d> DataStructure<'d> for KDTreeDataStructure<'d> {
    fn new(scene: &'d Scene<'d>) -> Self {
        debug!("Started building KD-Tree");
        let triangles: HashSet<&Triangle> = scene.triangles().collect();
        debug!("Cached triangles locally");

        let root = BVHNode::new(triangles);

        debug!("Generated tree: {}", root);

        Self { root }
    }

    fn intersects<'a>(&'a self, ray: &'a Ray) -> Option<Intersection<'a>> {
        None
    }
}
