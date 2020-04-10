use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;

mod builder;

use crate::postprocessors::PostProcessor;
pub use builder::RendererBuilder;
use crate::generator::Generator;

pub struct Renderer<'r> {
    generator: &'r dyn Generator,
    raytracer: &'r dyn RayTracer,
    shader: &'r dyn Shader,
    datastructure: &'r dyn DataStructure,
    postprocessor: &'r dyn PostProcessor,
}

impl<'r> Renderer<'r> {
    pub(self) fn new(
        generator: &'r dyn Generator,
        raytracer: &'r dyn RayTracer,
        shader: &'r dyn Shader,
        datastructure: &'r dyn DataStructure,
        postprocessor: &'r dyn PostProcessor,
    ) -> Self {
        Self {
            generator,
            raytracer,
            shader,
            datastructure,
            postprocessor,
        }
    }

    pub fn render(&self, camera: &Camera) -> OutputBuffer {
        let output = self
            .generator
            .generate_internal(self.raytracer, self.datastructure, self.shader, camera);

        self.postprocessor.process(output)
    }
}
