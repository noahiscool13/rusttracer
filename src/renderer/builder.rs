use crate::datastructure::DataStructure;
use crate::postprocessors::identity::IdentityPostProcessor;
use crate::postprocessors::PostProcessor;
use crate::raytracer::RayTracer;
use crate::renderer::Renderer;
use crate::shader::Shader;

pub struct RendererBuilder<'rb> {
    pub(self) datastructure: &'rb dyn DataStructure<'rb>,
}

pub struct RendererBuilderShader<'rb> {
    pub(self) datastructure: &'rb dyn DataStructure<'rb>,
    pub(self) shader: &'rb dyn Shader<'rb>,
}

pub struct RendererBuilderPostProcessor<'rb> {
    datastructure: &'rb dyn DataStructure<'rb>,
    raytracer: &'rb dyn RayTracer<'rb>,
    shader: &'rb dyn Shader<'rb>,
}

impl<'rb> RendererBuilder<'rb> {
    pub fn new(datastructure: &'rb dyn DataStructure<'rb>) -> Self {
        Self { datastructure }
    }

    pub fn with_shader(self, shader: &'rb dyn Shader<'rb>) -> RendererBuilderShader<'rb> {
        RendererBuilderShader {
            datastructure: self.datastructure,
            shader,
        }
    }
}

impl<'rb> RendererBuilderShader<'rb> {
    pub fn with_tracer(
        self,
        raytracer: &'rb dyn RayTracer<'rb>,
    ) -> RendererBuilderPostProcessor<'rb> {
        RendererBuilderPostProcessor {
            datastructure: self.datastructure,
            shader: self.shader,
            raytracer,
        }
    }
}

impl<'rb> RendererBuilderPostProcessor<'rb> {
    pub fn without_postprocessor(self) -> Renderer<'rb> {
        Renderer::new(
            self.datastructure,
            self.shader,
            self.raytracer,
            &IdentityPostProcessor,
        )
    }

    pub fn with_postprocessor(self, postprocessor: &'rb dyn PostProcessor) -> Renderer<'rb> {
        Renderer::new(
            self.datastructure,
            self.shader,
            self.raytracer,
            postprocessor,
        )
    }
}
