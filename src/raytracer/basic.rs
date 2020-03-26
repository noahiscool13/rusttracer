use crate::raytracer::RayTracer;
use crate::util::outputbuffer::OutputBuffer;
use crate::datastructure::DataStructure;
use crate::util::camera::Camera;
use crate::shader::Shader;

pub struct BasicRaytracer;

impl<'r, DS: DataStructure<'r>, S: Shader<'r, DS>> RayTracer<'r, DS, S> for BasicRaytracer {
    fn raytrace(&self, datastructure: &DS, shader: &S, camera: &Camera) -> OutputBuffer {
        let mut output = OutputBuffer::with_size(camera.width, camera.height);

        for x in 0..camera.width {
            for y in 0..camera.height {
                let ray = camera.generate_ray(x as f64, y as f64);
                output.set_at(x, y, shader.shade(ray,datastructure).into());
            }
        }
        output
    }
}