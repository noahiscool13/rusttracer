use crate::util::vector::Vector;
use crate::scene::Face;

#[derive(Clone, Debug)]
pub struct Triangle {
    pub a: Vector,
    pub b: Vector,
    pub c: Vector,

    pub face: Face,
}

impl Triangle {
    pub fn new(a: Vector, b: Vector, c: Vector, face: Face) -> Self {
        Self {a, b, c, face}
    }
}


#[cfg(test)]
mod tests {}