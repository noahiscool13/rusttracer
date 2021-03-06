use crate::generator::{Callback, Generator};
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;
use crossbeam::thread;
use log::info;

#[derive(Debug)]
pub struct CrossbeamGenerator {
    threads: usize,
}

impl CrossbeamGenerator {
    pub fn new(threads: usize) -> Self {
        Self { threads }
    }
}

impl Generator for CrossbeamGenerator {
    fn generate(&self, camera: &Camera, callback: &Callback) -> OutputBuffer {
        let mut output = OutputBuffer::with_size(camera.width, camera.height);

        thread::scope(|s| {
            let rows_per_thread = (camera.height / self.threads)
                + if camera.height % self.threads == 0 {
                    0
                } else {
                    1
                };

            for (index, chunk) in output.chunks_mut(rows_per_thread).enumerate() {
                let start_y = index * rows_per_thread;

                s.spawn(move |_| {
                    for y in start_y..(start_y + chunk.len()) {
                        let row = &mut chunk[y - start_y];

                        for (x, item) in row.iter_mut().enumerate().take(camera.width)  {
                            *item = callback(x, y);
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
