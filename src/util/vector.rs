use std::ops::{Add, Sub, Mul, Div};
use crate::util::color::Color;
use std::f64;
use rand::{Rng, thread_rng};

trait Clamp01 {
    fn clamp01(self) -> Self;
}


impl Clamp01 for f64 {
    fn clamp01(self) -> Self {
        self.min(1.).max(0.)
    }
}

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

    /// Makes a vector from one value, making the x, y and z coponent the same
    pub fn repeated(a: f64) -> Self {
        Self { x: a, y: a, z: a }
    }

    pub fn from_arr([a, b, c]: [f32; 3]) -> Self {
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

    pub fn rotated(&self, rotation: Vector) -> Vector {

        let nt = if rotation.x.abs() > rotation.y.abs() {
            Vector::new(rotation.z, 0f64, -rotation.x) / (rotation.x.powi(2) + rotation.z.powi(2)).sqrt()
        } else {
            Vector::new(0f64, -rotation.z, -rotation.y) / (rotation.y.powi(2) + rotation.z.powi(2)).sqrt()
        };

        let nb = rotation.cross(nt);

        let x = self.x * nb.x + self.y * rotation.x + self.z * nt.x;
        let y = self.x * nb.y + self.y * rotation.y + self.z * nt.y;
        let z = self.x * nb.z + self.y * rotation.z + self.z * nt.z;

        Vector::new(x,y,z)
    }

    pub fn point_on_hemisphere() -> Vector{

        let theta = thread_rng().gen::<f64>() * 2f64 * f64::consts::PI;
        let phi= (1f64-2f64*thread_rng().gen::<f64>()).acos();

        Vector::new(phi.sin()*theta.cos(),(phi.sin()*theta.sin()).abs(),phi.cos())
    }

    pub fn point_on_sphere() -> Vector{
        let theta = thread_rng().gen::<f64>() * 2f64 * f64::consts::PI;
        let phi= (2f64*thread_rng().gen::<f64>()-1f64).acos()-f64::consts::PI/2f64;

        Vector::new(phi.cos()*theta.cos(),phi.cos()*theta.sin(),phi.cos())
    }
}

impl Into<Color> for Vector {
    fn into(self) -> Color {
        Color {
            r: (self.x.clamp01() * 255.) as u8,
            g: (self.y.clamp01() * 255.) as u8,
            b: (self.z.clamp01() * 255.) as u8,
        }
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

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
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
    use crate::util::vector::Vector;
    use crate::util::color::Color;

    #[test]
    fn test_add() {
        let a = Vector::new(1f64, 2f64, 3f64);
        let b = Vector::new(5f64, 3f64, 2f64);

        let c = a + b;

        assert_eq!(c, Vector::new(6f64, 5f64, 5f64));
    }

    #[test]
    fn test_to_color_1() {
        let a: Vector = Vector::new(5., -5., 0.5);
        let c: Color = a.into();

        assert_eq!(c, Color {
            r: 255,
            g: 0,
            b: 127
        });
    }
}
