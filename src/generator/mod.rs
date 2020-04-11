use crate::datastructure::DataStructure;
use crate::raytracer::RayTracer;
use crate::shader::Shader;
use crate::util::camera::Camera;
use crate::util::outputbuffer::OutputBuffer;
use crate::util::vector::Vector;
use serde::export::fmt::Debug;

pub mod basic;
pub mod crossbeam;
pub mod rayon;

type Callback<'a> = (dyn Fn(usize, usize) -> Vector + Sync + 'a);

/// A generator is a struct that simply iterates over all x-y coordinates in the output image,
/// and calls generate(x, y) on it. After all pixels are iterated it collects all data
/// into an `Outputbuffer`.
///
/// This is important to be it's own subsystem because this iteration can be done in many ways
/// such as multithreaded, singlethreaded, or even spread over multiple machines.
pub trait Generator: Debug {
    fn generate_internal<'g>(
        &self,
        raytracer: &'g dyn RayTracer,
        datastructure: &'g dyn DataStructure,
        shader: &'g dyn Shader,
        camera: &Camera,
    ) -> OutputBuffer {
        self.generate(camera, &|x, y| {
            raytracer.raytrace(x, y, datastructure, shader, camera)
        })
    }

    fn generate<'g>(&self, camera: &Camera, callback: &Callback) -> OutputBuffer;
}
