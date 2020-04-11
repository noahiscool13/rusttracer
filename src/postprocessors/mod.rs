use crate::util::outputbuffer::OutputBuffer;
use serde::export::fmt::Debug;

pub mod gamma;
pub mod group;
pub mod identity;

/// After raytracing, a `PostProcessor` will be applied to the outputbuffer.
/// There are many options. If multiple postprocessor steps are required,
/// you can use a `PostProcessorGroup` which applies other postprocessors in order.
pub trait PostProcessor: Debug {
    fn process(&self, buffer: OutputBuffer) -> OutputBuffer;
}
