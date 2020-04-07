use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;

pub struct BasicRaytracer;

impl<'r> RayTracer<'r> for BasicRaytracer {
    fn raytrace(
        &self,
        datastructure: &'r dyn DataStructure<'r>,
        shader: &'r dyn Shader<'r>,
        camera: &Camera,
    ) -> OutputBuffer {
        let mut output = OutputBuffer::with_size(camera.width, camera.height);

        for x in 0..camera.width {
            for y in 0..camera.height {
                let ray = camera.generate_ray(x as f64, y as f64);
                output.set_at(x, y, shader.shade(&ray, datastructure).into());
            }
        }
        output
    }
}
