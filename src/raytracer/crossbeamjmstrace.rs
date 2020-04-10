use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;
use crate::util::rng::get_rng;
use crate::util::vector::Vector;
use crossbeam::thread;
use log::info;
use rand::Rng;

const SPP: usize = 1000;

pub struct CrossbeamJMSTracer {
    samples_per_pixel: usize,
}

impl CrossbeamJMSTracer {
    pub fn new(samples_per_pixel: usize) -> Self { Self { samples_per_pixel } }
}

impl RayTracer for CrossbeamJMSTracer {
    fn raytrace<'r>(
        &self,
        datastructure: &'r (dyn DataStructure + 'r),
        shader: &'r (dyn Shader + 'r),
        camera: &Camera,
    ) -> OutputBuffer {
        let mut output = OutputBuffer::with_size(camera.width, camera.height);

        thread::scope(|s| {
            let num_cpus = num_cpus::get();
            let rows_per_thread =
                (camera.height / num_cpus) + if camera.height % num_cpus == 0 { 0 } else { 1 };

            for (index, chunk) in output.chunks_mut(rows_per_thread).enumerate() {
                let start_y = index * rows_per_thread;

                s.spawn(move |_| {
                    for y in start_y..(start_y + chunk.len()) {
                        let row = &mut chunk[y - start_y];

                        for x in 0..camera.width {
                            let mut out = Vector::repeated(0f64);
                            for _ in 0..self.samples_per_pixel {
                                let ray = camera.generate_ray(
                                    x as f64 + get_rng(|mut r| r.gen::<f64>()),
                                    y as f64 + get_rng(|mut r| r.gen::<f64>()),
                                );

                                out = out + shader.shade(&ray, datastructure);
                            }
                            row[x] = (out / self.samples_per_pixel as f64).into();
                        }
                        info!("Finished row {}", y);
                    }
                });
            }
        })
            .expect("One of the threads in the threadpool has panicked!");

        output
    }
}
