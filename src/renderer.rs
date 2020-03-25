use crate::util::outputbuffer::OutputBuffer;
use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::util::camera::Camera;
use crate::scene::Scene;
use crate::shader::Shader;

pub struct Renderer<'r, DS: DataStructure<'r>, RT: RayTracer<'r, DS, S>, S: Shader<'r, DS>> {
    scene: &'r Scene,
    datastructure: DS,
    tracer: RT,
    shader: S,
}

impl<'r, DS: DataStructure<'r>, RT: RayTracer<'r, DS, S>, S: Shader<'r, DS>> Renderer<'r, DS, RT, S> {
    pub fn new(scene: &'r Scene) -> Self {
        Self {
            scene,
            datastructure: DS::new(scene),
            tracer: RT::new(),
            shader: Shader::new(scene)
        }
    }

    pub fn render(&self, camera: &Camera) -> OutputBuffer {
        self.tracer.raytrace(&self.datastructure, &self.shader, camera)
    }
}