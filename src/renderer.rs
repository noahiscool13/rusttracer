use crate::util::outputbuffer::OutputBuffer;
use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::util::camera::Camera;
use crate::shader::Shader;
use crate::scene::scene::Scene;

pub struct Renderer<'r, DS: DataStructure<'r>, RT: RayTracer<'r, DS, S>, S: Shader<'r>> {
    scene: &'r Scene<'r>,
    datastructure: DS,
    tracer: RT,
    shader: S,
}

impl<'r, DS: DataStructure<'r>, RT: RayTracer<'r, DS, S>, S: Shader<'r>> Renderer<'r, DS, RT, S> {
    pub fn new(scene: &'r Scene<'r>) -> Self {
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