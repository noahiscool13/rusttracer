use crate::postprocessors::PostProcessor;
use crate::util::outputbuffer::OutputBuffer;

/// This postprocessor returns the same image as it got in.
/// Useful if you don't want to postprocess at all.
#[derive(Debug)]
pub struct IdentityPostProcessor;

impl PostProcessor for IdentityPostProcessor {
    fn process(&self, buffer: OutputBuffer) -> OutputBuffer {
        buffer
    }
}
