use crate::datastructure::DataStructure;
use crate::postprocessors::identity::IdentityPostProcessor;
use crate::postprocessors::PostProcessor;
use crate::raytracer::RayTracer;
use crate::renderer::Renderer;
use crate::scene::scene::Scene;
use crate::shader::Shader;
use std::marker::PhantomData;

pub struct RendererBuilder<'rb> {
    scene: &'rb Scene<'rb>,
}

pub struct RendererBuilderDS<'rb, DS: DataStructure<'rb>> {
    pub(self) datastructure: DS,
    phantom: PhantomData<&'rb DS>,
}

pub struct RendererBuilderShader<'rb, DS: DataStructure<'rb>, S: Shader<'rb, DS>> {
    pub(self) datastructure: DS,
    pub(self) shader: S,
    phantom: PhantomData<&'rb DS>,
}

pub struct RendererBuilderPostProcessor<
    'r,
    DS: DataStructure<'r>,
    S: Shader<'r, DS>,
    RT: RayTracer<'r, DS, S>,
> {
    datastructure: DS,
    raytracer: RT,
    shader: S,
    phantom: PhantomData<&'r DS>,
}

impl<'rb> RendererBuilder<'rb> {
    pub fn new(scene: &'rb Scene<'rb>) -> Self {
        Self { scene }
    }

    pub fn with_datastructure<DS: DataStructure<'rb>>(self) -> RendererBuilderDS<'rb, DS> {
        RendererBuilderDS {
            datastructure: DS::new(self.scene),
            phantom: PhantomData,
        }
    }
}

impl<'rb, DS: DataStructure<'rb>> RendererBuilderDS<'rb, DS> {
    pub fn with_shader<S: Shader<'rb, DS>>(self, shader: S) -> RendererBuilderShader<'rb, DS, S> {
        RendererBuilderShader {
            datastructure: self.datastructure,
            shader,
            phantom: PhantomData,
        }
    }
}

impl<'rb, DS: DataStructure<'rb>, S: Shader<'rb, DS>> RendererBuilderShader<'rb, DS, S> {
    pub fn with_tracer<RT: RayTracer<'rb, DS, S>>(
        self,
        raytracer: RT,
    ) -> RendererBuilderPostProcessor<'rb, DS, S, RT> {
        RendererBuilderPostProcessor {
            datastructure: self.datastructure,
            shader: self.shader,
            raytracer,
            phantom: PhantomData,
        }
    }
}

impl<'rb, DS: DataStructure<'rb>, RT: RayTracer<'rb, DS, S>, S: Shader<'rb, DS>>
    RendererBuilderPostProcessor<'rb, DS, S, RT>
{
    pub fn without_postprocessor(self) -> Renderer<'rb, DS, S, RT> {
        Renderer::new(
            self.datastructure,
            self.shader,
            self.raytracer,
            &IdentityPostProcessor,
        )
    }

    pub fn with_postprocessor(
        self,
        postprocessor: &'rb dyn PostProcessor,
    ) -> Renderer<'rb, DS, S, RT> {
        Renderer::new(
            self.datastructure,
            self.shader,
            self.raytracer,
            postprocessor,
        )
    }
}
