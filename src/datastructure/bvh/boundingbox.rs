use crate::scene::triangle::Triangle;
use crate::util::vector::Vector;
use std::f64;

pub enum Axis {
    X(f64),
    Y(f64),
    Z(f64),
}

impl Axis {
    pub fn divide(&self, bounding_box: &BoundingBox, steps: usize) -> Vec<(BoundingBox, BoundingBox)> {
        match self {
            Axis::X(length) => (0..steps).map(|i| {
                bounding_box.split_at(Axis::X((1. / *length) * i as f64))
            }).collect(),
            Axis::Y(length) => (0..steps).map(|i| {
                bounding_box.split_at(Axis::Y((1. / *length) * i as f64))
            }).collect(),
            Axis::Z(length) => (0..steps).map(|i| {
                bounding_box.split_at(Axis::Z((1. / *length) * i as f64))
            }).collect(),
        }
    }
}

#[derive(Debug)]
pub struct BoundingBox {
    pub(super) min: Vector,
    pub(super) max: Vector,
}

impl BoundingBox {
    pub const EMPTY: BoundingBox = BoundingBox {
        min: Vector {
            x: f64::INFINITY,
            y: f64::INFINITY,
            z: f64::INFINITY,
        },
        max: Vector {
            x: f64::NEG_INFINITY,
            y: f64::NEG_INFINITY,
            z: f64::NEG_INFINITY,
        },
    };

    pub fn new(min: Vector, max: Vector) -> Self {
        Self { min, max }
    }

    pub fn from_triangle(triangle: &Triangle) -> Self {
        Self::EMPTY
            .include_point(triangle.a())
            .include_point(triangle.b())
            .include_point(triangle.c())
    }

    pub fn merge(&self, other: &Self) -> Self {
        self.include_point(other.min).include_point(other.max)
    }

    pub fn include_point(&self, point: Vector) -> Self {
        Self {
            min: self.min.min(&point),
            max: self.max.max(&point),
        }
    }

    pub fn from_triangles<'a>(triangles: impl Iterator<Item=&'a Triangle<'a>>) -> Self {
        let mut curr = Self::EMPTY;
        for i in triangles {
            curr.merge(&BoundingBox::from_triangle(i));
        }

        curr
    }

    pub fn size(&self) -> Vector {
        let x = self.max.x - self.min.x;
        let y = self.max.y - self.min.y;
        let z = self.max.z - self.min.z;

        Vector::new(x, y, z)
    }

    pub fn surface_area(&self) -> f64 {
        let size = self.size();
        let surface_top = size.x*size.z;
        let surface_front = size.x*size.y;
        let surface_side = size.y*size.z;

        2.*(surface_top+surface_front+surface_side)
    }

    pub fn cost(&self, numtriangles: usize) -> f64 {
        self.surface_area()*numtriangles as f64
    }

    pub fn contains(&self, triangle: &Triangle) -> bool{
        true
    }

    pub fn split_at(&self, axis: Axis) -> (BoundingBox, BoundingBox) {
        match axis {
            Axis::X(i) => (BoundingBox {
                min: self.min,
                max: Vector::new(self.min.x + i, self.max.y, self.max.z),
            }, BoundingBox {
                min: Vector::new(self.min.x + i, self.max.y, self.max.z),
                max: self.max,
            }),
            Axis::Y(i) => (BoundingBox {
                min: self.min,
                max: Vector::new(self.max.x, self.min.y + i, self.max.z),
            }, BoundingBox {
                min: Vector::new(self.max.x, self.min.y + i, self.max.z),
                max: self.max,
            }),
            Axis::Z(i) => (BoundingBox {
                min: self.min,
                max: Vector::new(self.max.x, self.max.y, self.min.z + i),
            }, BoundingBox {
                min: Vector::new(self.max.x, self.max.y, self.min.z + i),
                max: self.max,
            }),
        }
    }

    pub fn longest_axis(&self) -> Axis {
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let dz = self.max.z - self.min.z;

        if dx > dy && dx > dz {
            Axis::X(dx)
        } else if dx > dy && dx <= dz {
            Axis::Z(dz)
        } else if dx <= dy && dy > dz {
            Axis::Y(dy)
        } else {
            Axis::Z(dz)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::datastructure::bvh::boundingbox::BoundingBox;
    use crate::scene::scene::Mesh;
    use crate::scene::triangle::Triangle;
    use crate::util::vector::Vector;

    #[test]
    fn test_create() {
        let bb = BoundingBox::new(Vector::new(0., 0., 0.), Vector::new(1., 1., 1.));
        assert_eq!(bb.min, Vector::new(0., 0., 0.));
        assert_eq!(bb.max, Vector::new(1., 1., 1.));
    }

    #[test]
    fn test_include_point() {
        let bb = BoundingBox::EMPTY;

        let ibb = bb
            .include_point(Vector::new(0., 0., 0.))
            .include_point(Vector::new(1., 1., 1.));

        assert_eq!(ibb.min, Vector::new(0., 0., 0.));
        assert_eq!(ibb.max, Vector::new(1., 1., 1.));
    }

    #[test]
    fn test_merge() {
        let bb1 = BoundingBox::new(Vector::new(-5., -2., 0.), Vector::new(7., 4., 4.));
        let bb2 = BoundingBox::new(Vector::new(8., -7., -2.), Vector::new(14., 2., 8.));

        let bb3 = bb1.merge(&bb2);

        assert_eq!(bb3.min, Vector::new(-5., -7., -2.));
        assert_eq!(bb3.max, Vector::new(14., 4., 8.));
    }
}
