use crate::postprocessors::PostProcessor;
use crate::util::outputbuffer::OutputBuffer;

/// Post Processes an OutputBuffer by applying multiple other postprocessors.
/// Will apply Processors based on the order in which they are
/// added with `add_postprocessor`
pub struct PostProcessorGroup<'p> {
    processors: Vec<&'p dyn PostProcessor>,
}

impl<'p> PostProcessorGroup<'p> {
    pub fn new() -> Self {
        Self { processors: vec![] }
    }

    pub fn add_postprocessor(&mut self, postprocessor: &'p dyn PostProcessor) {
        self.processors.push(postprocessor)
    }
}

impl<'p> PostProcessor for PostProcessorGroup<'p> {
    fn process(&self, buffer: OutputBuffer) -> OutputBuffer {
        let mut res = buffer;

        for processor in &self.processors {
            res = processor.process(res);
        }

        res
    }
}
