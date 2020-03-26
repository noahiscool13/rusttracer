use crate::raytracer::RayTracer;
use crate::util::outputbuffer::OutputBuffer;
use crate::datastructure::DataStructure;
use crate::util::camera::Camera;
use crate::shader::Shader;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use crate::util::vector::Vector;
use rand::{thread_rng, Rng};

const SPP : usize = 15;

pub struct JMSTracer;

impl<'r, DS: DataStructure<'r> + Sync, S: Shader<'r, DS> + Sync> RayTracer<'r, DS, S> for JMSTracer {
    fn raytrace(&self, datastructure: &DS, shader: &S, camera: &Camera) -> OutputBuffer {
        let mut output = OutputBuffer::with_size(camera.width, camera.height);



        output.par_iter_mut().enumerate().for_each(|(y, row)| {
            for x in 0..camera.width {
                let mut out = Vector::repeated(0f64);
                for _ in 0..SPP {
                    let ray = camera.generate_ray(x as f64 + thread_rng().gen::<f64>(), y as f64 + thread_rng().gen::<f64>());

                    out = out + shader.shade(ray, datastructure);

                }
                row[x] = (out / SPP as f64).into();
            }

            println!("Finished row {}", y);
        });

        output
    }
}