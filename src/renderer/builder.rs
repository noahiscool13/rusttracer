use crate::datastructure::DataStructure;
use crate::postprocessors::identity::IdentityPostProcessor;
use crate::postprocessors::PostProcessor;
use crate::raytracer::RayTracer;
use crate::renderer::Renderer;
use crate::shader::Shader;

pub struct RendererBuilder<'a> {
    pub(self) datastructure: &'a (dyn DataStructure + 'a),
}

pub struct RendererBuilderShader<'a> {
    pub(self) datastructure: &'a (dyn DataStructure + 'a),
    pub(self) shader: &'a (dyn Shader + 'a),
}

pub struct RendererBuilderPostProcessor<'a> {
    datastructure: &'a (dyn DataStructure + 'a),
    raytracer: &'a (dyn RayTracer + 'a),
    shader: &'a (dyn Shader + 'a),
}

impl<'a> RendererBuilder<'a> {
    pub fn new(datastructure: &'a (dyn DataStructure + 'a)) -> Self {
        Self { datastructure }
    }

    pub fn with_shader(self, shader: &'a (dyn Shader + 'a)) -> RendererBuilderShader<'a> {
        RendererBuilderShader {
            datastructure: self.datastructure,
            shader,
        }
    }
}

impl<'a> RendererBuilderShader<'a> {
    pub fn with_tracer(
        self,
        raytracer: &'a (dyn RayTracer + 'a),
    ) -> RendererBuilderPostProcessor<'a> {
        RendererBuilderPostProcessor {
            datastructure: self.datastructure,
            shader: self.shader,
            raytracer,
        }
    }
}

impl<'a> RendererBuilderPostProcessor<'a> {
    pub fn without_postprocessor(self) -> Renderer<'a> {
        Renderer::new(
            self.datastructure,
            self.shader,
            self.raytracer,
            &IdentityPostProcessor,
        )
    }

    pub fn with_postprocessor(self, postprocessor: &'a dyn PostProcessor) -> Renderer<'a> {
        Renderer::new(
            self.datastructure,
            self.shader,
            self.raytracer,
            postprocessor,
        )
    }
}
