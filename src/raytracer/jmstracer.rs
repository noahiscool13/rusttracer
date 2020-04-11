use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;
use crate::util::rng::get_rng;
use crate::util::vector::Vector;
use rand::Rng;

#[derive(Debug)]
pub struct JMSTracer {
    samples_per_pixel: usize,
}

impl JMSTracer {
    pub fn new(samples_per_pixel: usize) -> Self { Self { samples_per_pixel } }
}

impl RayTracer for JMSTracer {
    fn raytrace<'r>(
        &self,
        x: usize,
        y: usize,
        datastructure: &'r (dyn DataStructure + 'r),
        shader: &'r (dyn Shader + 'r),
        camera: &Camera,
    ) -> Vector {

        let mut out = Vector::repeated(0f64);
        for _ in 0..self.samples_per_pixel {
            let ray = camera.generate_ray(
                x as f64 + get_rng(|mut r| r.gen::<f64>()),
                y as f64 + get_rng(|mut r| r.gen::<f64>()),
            );

            out = out + shader.shade(&ray, datastructure);
        }


        out / self.samples_per_pixel as f64
    }
}
