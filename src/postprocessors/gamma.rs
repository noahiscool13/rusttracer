use crate::postprocessors::PostProcessor;
use crate::util::outputbuffer::OutputBuffer;

/// This postprocessor returns the same image as it got in.
/// Useful if you don't want to postprocess at all.
#[derive(Debug)]
pub struct Gamma;

impl PostProcessor for Gamma {
    fn process(&self, buffer: OutputBuffer) -> OutputBuffer {
        let height = buffer.len();
        let width = if height > 0 { buffer[0].len() } else { 0 };

        let mut new_buffer = OutputBuffer::new();

        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                row.push(buffer[y][x].gamma(0.5f64));
            }
            new_buffer.push(row);
        }

        new_buffer
    }
}
