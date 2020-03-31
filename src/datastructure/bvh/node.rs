use crate::datastructure::bvh::boundingbox::{Axis, BoundingBox};
use crate::scene::triangle::Triangle;
use crate::util::vector::Vector;
use log::debug;
use std::collections::HashSet;
use std::f32::MAX;
use std::fmt::{Display, Formatter, Error, Debug};
use core::fmt;

// Implementation helped by the pseudocode given by Arend van beelen in his bachelor's thesis on page 28:
// http://liacs.leidenuniv.nl/assets/Bachelorscripties/2006-06arendvanbeelen.pdf

pub(super) struct BVHNode<'d> {

    bounding_box: BoundingBox,

    left: Option<Box<BVHNode<'d>>>,
    right: Option<Box<BVHNode<'d>>>,

    triangles: HashSet<&'d Triangle<'d>>,
}

impl<'d> Display for BVHNode<'d> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.print(f, 0)
    }
}

impl<'d> BVHNode<'d> {

    fn print(&self, f: &mut Formatter<'_>, depth: usize) -> fmt::Result {
        writeln!(f, "node [{:?}]:", self.triangles.len())?;
        if let Some(l) = &self.left{
            write!(f, "{}", "\t".repeat(depth))?;
            l.print(f, depth+1)?;
        }
        if let Some(r) = &self.right{
            write!(f, "{}", "\t".repeat(depth))?;
            r.print(f, depth+1)?;
        }

        Ok(())
    }

    pub fn new(triangles: HashSet<&'d Triangle<'d>>) -> Self {
        debug!("Creating new KD Tree with {} triangles", triangles.len());

        Self::new_internal(triangles, 0)
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

    const MIN_TRIANGLE_PER_LEAF: usize = 5;
    const MAX_TRIANGLE_PER_LEAF: usize = 20;
    const MAX_RECURSION_DEPTH: usize = 20;

    fn new_internal(
        triangles: HashSet<&'d Triangle<'d>>,
        depth: usize,
    ) -> Self {
        debug!(
            "Recursing (depth={}) with {} triangles",
            depth,
            triangles.len()
        );
        let bounding_box = BoundingBox::from_triangles(triangles.iter().cloned());

        if triangles.len() < Self::MIN_TRIANGLE_PER_LEAF
            || (depth > Self::MAX_RECURSION_DEPTH && triangles.len() < Self::MAX_TRIANGLE_PER_LEAF)
        {
            Self {
                triangles,
                bounding_box,
                left: None,
                right: None,
            }
        } else {
            let average = Self::average(triangles.iter().cloned());

            // TODO: select axis based on depth?
            let longest_axis = bounding_box.longest_axis();

            debug!("Choosing midpoint as {:?}", average);

            let mut right_triangles = HashSet::new();
            let mut left_triangles = HashSet::new();

            for triangle in &triangles {
                for vertex in [triangle.a(), triangle.b(), triangle.c()].iter() {
                    match &longest_axis {
                        Axis::X if average.x >= triangle.midpoint().x => right_triangles.insert(*triangle),
                        Axis::X /* else */ => left_triangles.insert(*triangle),

                        Axis::Y if average.y >= triangle.midpoint().y => right_triangles.insert(*triangle),
                        Axis::Y /* else */ => left_triangles.insert(*triangle),

                        Axis::Z if average.z >= triangle.midpoint().z => right_triangles.insert(*triangle),
                        Axis::Z /* else */ => left_triangles.insert(*triangle),
                    };
                }
            }

            debug!("Making left size: {}", left_triangles.len());
            debug!("Making right size: {}", right_triangles.len());

            // if left_triangles.len() == 0 && right_triangles.len() > 0
            let similar = left_triangles
                .intersection(&right_triangles)
                .collect::<Vec<_>>()
                .len();
            debug!("similar triangles in left and right: {}", similar);

            Self {
                bounding_box,
                left: Some(Box::new(BVHNode::new_internal(left_triangles, depth + 1))),
                right: Some(Box::new(BVHNode::new_internal(right_triangles, depth + 1))),
                triangles,
            }
        }
    }
}
