use std::ops::{Add, Sub, Mul};

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}


impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_arr([a,b,c]: [f32; 3]) -> Self {
        Self::new(a as f64, b as f64, c as f64)
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length2(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        if length > 0f64 {
            self.x /= length;
            self.y /= length;
            self.z /= length;
        }
    }

    pub fn unit(&self) -> Vector {
        let length = self.length();
        Vector::new(
            self.x / length,
            self.y / length,
            self.z / length,
        )
    }

    // TODO rotations
    pub fn rotated(&self, rotation: Vector) -> Vector {
        rotation
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}


impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}


impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::vector::Vector;

    #[test]
    fn test_add() {
        let a = Vector::new(1f64, 2f64, 3f64);
        let b = Vector::new(5f64, 3f64, 2f64);

        let c = a + b;

        assert_eq!(c, Vector::new(6f64, 5f64, 5f64));
    }
}
