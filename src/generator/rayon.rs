use crate::generator::{Generator, Callback};
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;
use crate::util::vector::Vector;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use log::info;
use rayon::ThreadPoolBuilder;

#[derive(Debug)]
pub struct RayonGenerator {
    threads: usize
}

impl RayonGenerator {
    pub fn new(threads: usize) -> Self { Self { threads: threads } }
}

impl Generator for RayonGenerator {
    fn generate<'g>(&self, camera: &Camera, callback: &Callback) -> OutputBuffer {
        // Todo: error handling
        ThreadPoolBuilder::new()
            .num_threads(self.threads)
            .build_global()
            .unwrap();

        let mut output = OutputBuffer::with_size(camera.width, camera.height);

        output.par_iter_mut().enumerate().for_each(|(y, row)| {
            for x in 0..camera.width {

                row[x] = callback(x, y);
            }

            info!("Finished row {}", y);
        });

        output
    }
}