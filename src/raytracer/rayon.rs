use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

pub struct RayonRaytracer;

impl RayTracer for RayonRaytracer {
    fn raytrace<'r>(
        &self,
        datastructure: &'r (dyn DataStructure + 'r),
        shader: &'r (dyn Shader + 'r),
        camera: &Camera,
    ) -> OutputBuffer {
        let mut output = OutputBuffer::with_size(camera.width, camera.height);

        output.par_iter_mut().enumerate().for_each(|(y, row)| {
            for x in 0..camera.width {
                let ray = camera.generate_ray(x as f64, y as f64);
                row[x] = shader.shade(&ray, datastructure).into();
            }
        });

        output
    }
}
