use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::vector::Vector;

#[derive(Debug)]
pub struct BasicRaytracer;

impl RayTracer for BasicRaytracer {
    fn raytrace<'r>(
        &self,
        x: usize,
        y: usize,
        datastructure: &'r (dyn DataStructure + 'r),
        shader: &'r (dyn Shader + 'r),
        camera: &Camera,
    ) -> Vector {
        let ray = camera.generate_ray(x as f64, y as f64);
        shader.shade(&ray, datastructure)
    }
}
