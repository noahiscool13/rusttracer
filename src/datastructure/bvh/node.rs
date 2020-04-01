use crate::datastructure::bvh::boundingbox::{Axis, BoundingBox};
use crate::scene::triangle::Triangle;
use crate::util::vector::Vector;
use core::fmt;
use log::debug;
use std::fmt::{Debug, Display, Error, Formatter};

pub enum BVHNode<'d> {
    Leaf {
        bounding_box: BoundingBox,
        triangles: Vec<&'d Triangle<'d>>,
    },
    Node {
        bounding_box: BoundingBox,

        left: Box<BVHNode<'d>>,
        right: Box<BVHNode<'d>>,
    },
}

impl<'d> Display for BVHNode<'d> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.print(f, 0)
    }
}

impl<'d> BVHNode<'d> {
    fn print(&self, f: &mut Formatter<'_>, depth: usize) -> fmt::Result {
        match self {
            BVHNode::Leaf { bounding_box,triangles } => {
                write!(f, "{}", "\t".repeat(depth))?;
                writeln!(
                    f,
                    "leaf node with {} triangles [{:?}]:",
                    triangles.len(),
                    triangles
                )?;
                writeln!(f, "Bounding box: {:#?}", bounding_box)?;
            }
            BVHNode::Node { left, right, bounding_box } => {
                write!(f, "{}", "\t".repeat(depth))?;
                writeln!(f, "Bounding box: {:#?}", bounding_box)?;
                left.print(f, depth + 1)?;
                right.print(f, depth + 1)?;
            }
        }

        Ok(())
    }

    pub fn new(triangles: Vec<&'d Triangle<'d>>) -> Self {
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

    fn divide_triangles_over_boundingboxes<'a>(
        (leftbox, rightbox): (&BoundingBox, &BoundingBox),
        triangles: &Vec<&'a Triangle<'a>>,
    ) -> (Vec<&'a Triangle<'a>>, Vec<&'a Triangle<'a>>) {
        let mut leftset = Vec::new();
        let mut rightset = Vec::new();

        for i in triangles {
            if leftbox.contains(i) {
                leftset.push(*i);
            }
            if rightbox.contains(i) {
                rightset.push(*i);
            }
        }

        (leftset, rightset)
    }

    fn new_internal(
        triangles: Vec<&'d Triangle<'d>>,
        bounding_box: BoundingBox,
        depth: usize,
    ) -> Self {
        if triangles.len() == 0 {
            return BVHNode::Leaf {
                bounding_box: BoundingBox::EMPTY,
                triangles,
            };
        }

        let longest_axis = bounding_box.longest_axis();

        struct State<'s> {
            leftbox: BoundingBox,
            rightbox: BoundingBox,
            leftset: Vec<&'s Triangle<'s>>,
            rightset: Vec<&'s Triangle<'s>>,

            totalcost: f64,
        }

        let mut smallest: Option<State> = None;

        for (leftbox, rightbox) in longest_axis.divide(&bounding_box, 16) {
            let (leftset, rightset) =
                Self::divide_triangles_over_boundingboxes((&leftbox, &rightbox), &triangles);

            let leftcost = leftbox.cost(leftset.len());
            let rightcost = rightbox.cost(rightset.len());
            let totalcost = leftcost + rightcost;

            if let Some(s) = smallest.as_ref() {
                if totalcost < s.totalcost {
                    smallest = Some(State {
                        leftbox,
                        rightbox,
                        leftset,
                        rightset,
                        totalcost,
                    })
                }
            } else {
                smallest = Some(State {
                    leftbox,
                    rightbox,
                    leftset,
                    rightset,
                    totalcost,
                });
            }
        }

        // Can't fail because smallest is set in the first iteration of the loop.
        let smallest = smallest.unwrap();
        let current_cost = bounding_box.cost(triangles.len());

        debug!("Smallest possible split cost: {}", smallest.totalcost);
        debug!("Parent split cost: {}", current_cost);

        if smallest.totalcost >= current_cost {
            BVHNode::Leaf {
                bounding_box,
                triangles,
            }
        } else {
            BVHNode::Node {
                bounding_box,
                left: Box::new(Self::new_internal(
                    smallest.leftset,
                    smallest.leftbox,
                    depth + 1,
                )),
                right: Box::new(Self::new_internal(
                    smallest.rightset,
                    smallest.rightbox,
                    depth + 1,
                )),
            }
        }
    }
}
