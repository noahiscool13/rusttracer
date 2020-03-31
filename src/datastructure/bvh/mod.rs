use crate::datastructure::intersection::Intersection;
use crate::datastructure::bvh::boundingbox::BoundingBox;
use crate::datastructure::bvh::node::BVHNode;
use crate::datastructure::DataStructure;
use crate::scene::scene::Scene;
use crate::scene::triangle::Triangle;
use crate::util::ray::Ray;
use log::debug;
use std::collections::HashSet;
use crate::datastructure::bvh::boxintersection::BoxIntersection;

mod boundingbox;
mod node;
mod boxintersection;

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

pub fn intersects_boundingbox(
    boundingbox: &BoundingBox,
    ray: &Ray,
    triangle: &Triangle,
) -> Option<BoxIntersection> {
    let mut tmin = (boundingbox.min.x - ray.origin.x) / ray.direction.x;
    let mut tmax = (boundingbox.max.x - ray.origin.x) / ray.direction.x;

    if tmin > tmax {
        let (tmin, tmax) = (tmax, tmin);
    }

    let tymin = (boundingbox.min.y - ray.origin.y) / ray.direction.y;
    let tymax = (boundingbox.max.y - ray.origin.y) / ray.direction.y;

    if tymin>tymax{
        let (tymin, tymax) = (tymax, tymin);
    }

    if (tmin>tymax) || (tymin>tmax) {
        return None;
    }

    if tymin>tmin {
        tmin = tymin;
    }

    if tymax<tmax{
        tmax = tymax;
    }

    let tzmin = (boundingbox.min.z - ray.origin.z) / ray.direction.z;
    let tzmax = (boundingbox.max.z - ray.origin.z) / ray.direction.z;

    if tzmin>tzmax{
        let (tzmin, tzmax) = (tzmax, tzmin);
    }

    if (tmin>tzmax) || (tzmin>tmax) {
        return None;
    }

    if tyzin>tmin {
        tmin = tzmin;
    }

    if tzmax<tmax{
        tmax = tzmax;
    }

    let t = tmin.min(tmax);

    Some(BoxIntersection {
        t,
        ray,
        boundingbox,
    })
}
