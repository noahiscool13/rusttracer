use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;
use crate::util::vector::Vector;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use log::info;

const SPP: usize = 30000;

pub struct MSTracer;

impl RayTracer for MSTracer {
    fn raytrace<'r>(
        &self,
        datastructure: &'r (dyn DataStructure + 'r),
        shader: &'r (dyn Shader + 'r),
        camera: &Camera,
    ) -> OutputBuffer {
        let mut output = OutputBuffer::with_size(camera.width, camera.height);

        output.par_iter_mut().enumerate().for_each(|(y, row)| {
            for x in 0..camera.width {
                let mut out = Vector::repeated(0f64);
                for _ in 0..SPP {
                    let ray = camera.generate_ray(x as f64, y as f64);
                    out = out + shader.shade(&ray, datastructure);
                }
                row[x] = (out / SPP as f64).into();
            }

            info!("Finished row {}", y);
        });

        output
    }
}
