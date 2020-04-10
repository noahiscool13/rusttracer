use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;

mod builder;

use crate::postprocessors::PostProcessor;
pub use builder::RendererBuilder;

pub struct Renderer<'r> {
    datastructure: &'r (dyn DataStructure + 'r),
    tracer: &'r (dyn RayTracer + 'r),
    shader: &'r (dyn Shader + 'r),
    postprocessor: &'r dyn PostProcessor,
}

impl<'r> Renderer<'r> {
    pub(self) fn new(
        datastructure: &'r (dyn DataStructure + 'r),
        shader: &'r (dyn Shader + 'r),
        tracer: &'r (dyn RayTracer + 'r),
        postprocessor: &'r dyn PostProcessor,
    ) -> Self {
        Self {
            datastructure,
            tracer,
            shader,
            postprocessor,
        }
    }

    pub fn render(&self, camera: &Camera) -> OutputBuffer {
        let output = self
            .tracer
            .raytrace(self.datastructure, self.shader, camera);

        self.postprocessor.process(output)
    }
}
