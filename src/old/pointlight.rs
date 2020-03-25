use crate::vector::Vector;
use crate::color::Color;

pub struct PointLight{
    pos: Vector,
    col: Color,
    brightness: f64
}