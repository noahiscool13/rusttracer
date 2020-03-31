use crate::datastructure::bvh::boundingbox::{Axis, BoundingBox};
use crate::scene::triangle::Triangle;
use crate::util::vector::Vector;
use log::debug;
use std::collections::HashSet;
use std::f32::MAX;
use std::fmt::{Display, Formatter, Error, Debug};
use core::fmt;
use crate::datastructure::bvh::node::BVHNode::Empty;

pub(super) enum BVHNode<'d> {
    Leaf {
        bounding_box: BoundingBox,
        triangles: HashSet<&'d Triangle<'d>>,
    },
    Node {
        bounding_box: BoundingBox,

        left: Box<BVHNode<'d>>,
        right: Box<BVHNode<'d>>,
    },

    Empty,
}

// impl<'d> Display for BVHNode<'d> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         self.print(f, 0)
//     }
// }

impl<'d> BVHNode<'d> {


    // fn print(&self, f: &mut Formatter<'_>, depth: usize) -> fmt::Result {
    //     writeln!(f, "node [{:?}]:", self.triangles.len())?;
    //     if let Some(l) = &self.left{
    //         write!(f, "{}", "\t".repeat(depth))?;
    //         l.print(f, depth+1)?;
    //     }
    //     if let Some(r) = &self.right{
    //         write!(f, "{}", "\t".repeat(depth))?;
    //         r.print(f, depth+1)?;
    //     }
    //
    //     Ok(())
    // }

    pub fn new(triangles: HashSet<&'d Triangle<'d>>) -> Self {
        debug!("Creating new KD Tree with {} triangles", triangles.len());

        let bb = BoundingBox::from_triangles(triangles.iter().cloned());

        Self::new_internal(triangles, bb, 0)
    }

    fn average<'a>(triangles: impl Iterator<Item = &'a Triangle<'a>>) -> Vector {
        let mut total = Vector::default();
        let mut length = 0.;
        for i in triangles {
            total += i.midpoint();
            length += 1.;
        }

        total / length
    }


    fn new_internal(
        triangles: HashSet<&'d Triangle<'d>>,
        boundingbox: BoundingBox,
        depth: usize,
    ) -> Self {

        let longest_axis = boundingbox.longest_axis();

        for (left, right) in longest_axis.divide(&boundingbox, 16) {

        }

        BVHNode::Empty
    }
}
