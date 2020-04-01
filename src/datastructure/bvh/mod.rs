use crate::datastructure::bvh::boundingbox::BoundingBox;
use crate::datastructure::bvh::boxintersection::BoxIntersection;
use crate::datastructure::bvh::node::BVHNode;
use crate::datastructure::intersection::Intersection;
use crate::datastructure::DataStructure;
use crate::scene::scene::Scene;
use crate::scene::triangle::Triangle;
use crate::util::ray::Ray;
use log::debug;
use std::collections::HashSet;
use crate::util::vector::Vector;

mod boundingbox;
mod boxintersection;
mod node;

pub struct KDTreeDataStructure<'d> {
    root: BVHNode<'d>,
}

impl<'d> KDTreeDataStructure<'d> {

    fn intersect_internal<'a>(ray: &'a Ray, node: &BVHNode) -> Option<Intersection<'a>> {
        match node {
            BVHNode::Leaf {bounding_box, triangles} => {

            }
            BVHNode::Node {bounding_box, left, right} => {
                let dist_l = intersects_bhv(&left, ray);
                let dist_r = intersects_bhv(&right, ray);

                match (dist_l, dist_r) {
                    (None, None) => return None,
                    (Some(i), None) => return Self::intersect_internal(ray, left),
                    (None, Some(i)) => return Self::intersect_internal(ray, right),
                    (Some(left_intersection),Some(right_intersection)) => {
                        if left_intersection.t < right_intersection.t {
                            let hit = Self::intersect_internal(ray, left);
                            if let Some(hit_int) = hit {
                                if point_in_bhv(hit_int.hit_pos(),left) {
                                    return Some(hit_int);
                                }
                            }
                            return Self::intersect_internal(ray, right);
                        }

                        let hit = Self::intersect_internal(ray, right);
                        if let Some(hit_int) = hit {
                            if point_in_bhv(hit_int.hit_pos(),right) {
                                return Some(hit_int);
                            }
                        }
                        return Self::intersect_internal(ray, left);
                    }

                }
            }
        }

    None
    }

}

impl<'d> DataStructure<'d> for KDTreeDataStructure<'d> {
    fn new(scene: &'d Scene<'d>) -> Self {
        debug!("Started building KD-Tree");
        let triangles: HashSet<&Triangle> = scene.triangles().collect();
        debug!("Cached triangles locally");

        let root = BVHNode::new(triangles);
        debug!("{}", root);

        Self { root }
    }

    fn intersects<'a>(&'a self, ray: &'a Ray) -> Option<Intersection<'a>> {
        Self::intersect_internal(ray, &self.root)
    }
}

pub fn point_in_bhv(point: Vector, node: &BVHNode) -> bool {
    match node {
        BVHNode::Leaf {bounding_box, triangles: _} => {
            point_in_boundingbox(point,bounding_box)
        }
        BVHNode::Node {bounding_box, left: _, right:_} => {
            point_in_boundingbox(point,bounding_box)
        }
    }
}


pub fn point_in_boundingbox(point: Vector, boundingbox: &BoundingBox) -> bool {
    if point.x< boundingbox.min.x{
        return false;
    }
    if point.y< boundingbox.min.y{
        return false;
    }
    if point.z< boundingbox.min.z{
        return false;
    }

    if point.x> boundingbox.max.x{
        return false;
    }
    if point.y> boundingbox.max.y{
        return false;
    }
    if point.z> boundingbox.max.z{
        return false;
    }

    return true;
}

pub fn intersects_bhv<'a>(
    node: &'a BVHNode,
    ray: &'a Ray,
) -> Option<BoxIntersection<'a>> {
    match node {
        BVHNode::Leaf {bounding_box, triangles: _} => {
            intersects_boundingbox(bounding_box,ray)
        }
        BVHNode::Node {bounding_box, left: _, right:_} => {
            intersects_boundingbox(bounding_box,ray)
        }
    }

}

pub fn intersects_boundingbox<'a>(
    boundingbox: &'a BoundingBox,
    ray: &'a Ray,
) -> Option<BoxIntersection<'a>> {
    let mut tmin = (boundingbox.min.x - ray.origin.x) / ray.direction.x;
    let mut tmax = (boundingbox.max.x - ray.origin.x) / ray.direction.x;

    let (mut tmin, mut tmax) = if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    };

    let tymin = (boundingbox.min.y - ray.origin.y) / ray.direction.y;
    let tymax = (boundingbox.max.y - ray.origin.y) / ray.direction.y;

    let (mut tymin,mut tymax) = if tymin > tymax {
        (tymax, tymin)
    } else {
        (tymin, tymax)
    };

    if (tmin > tymax) || (tymin > tmax) {
        return None;
    }

    if tymin > tmin {
        tmin = tymin;
    }

    if tymax < tmax {
        tmax = tymax;
    }

    let tzmin = (boundingbox.min.z - ray.origin.z) / ray.direction.z;
    let tzmax = (boundingbox.max.z - ray.origin.z) / ray.direction.z;

    if tzmin > tzmax {
        let (tzmin, tzmax) = (tzmax, tzmin);
    }

    if (tmin > tzmax) || (tzmin > tmax) {
        return None;
    }

    if tzmin > tmin {
        tmin = tzmin;
    }

    if tzmax < tmax {
        tmax = tzmax;
    }

    let t = tmin.min(tmax);

    Some(BoxIntersection {
        t,
        ray,
        boundingbox,
    })
}
