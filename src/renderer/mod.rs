use crate::util::outputbuffer::OutputBuffer;
use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::util::camera::Camera;
use crate::shader::Shader;

mod builder;

pub use builder::RendererBuilder;
use std::marker::PhantomData;
use crate::postprocessors::PostProcessor;

pub struct Renderer<'r, DS: DataStructure<'r>, S: Shader<'r, DS>, RT: RayTracer<'r, DS, S>> {
    datastructure: DS,
    tracer: RT,
    shader: S,
    postprocessor: &'r dyn PostProcessor,
    phantom: PhantomData<&'r DS>,
}

impl<'r, DS: DataStructure<'r>, RT: RayTracer<'r, DS, S>, S: Shader<'r, DS>> Renderer<'r, DS, S, RT> {
    pub(self) fn new(datastructure: DS, shader: S, tracer: RT, postprocessor: &'r dyn PostProcessor) -> Self {
        Self {
            datastructure,
            tracer,
            shader,
            postprocessor,
            phantom: PhantomData
        }
    }

    pub fn render(&self, camera: &Camera) -> OutputBuffer {
        let output = self.tracer.raytrace(&self.datastructure, &self.shader, camera);

        self.postprocessor.process(output)
    }
}