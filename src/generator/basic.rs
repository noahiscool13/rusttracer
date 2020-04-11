use crate::generator::{Generator, Callback};
use crate::raytracer::RayTracer;
use crate::util::outputbuffer::OutputBuffer;
use crate::util::vector::Vector;
use crate::util::camera::Camera;

#[derive(Debug)]
pub struct BasicGenerator;

impl Generator for BasicGenerator {
    fn generate(&self, camera: &Camera, callback: &Callback) -> OutputBuffer {
        let mut output = OutputBuffer::with_size(camera.width, camera.height);

        for x in 0..camera.width {
            for y in 0..camera.height {
                let res = callback(x, y);
                output.set_at(x, y, res);
            }
        }

        output
    }
}