use crate::config::{Config, DatastructureConfig, ShaderConfig, RaytracerConfig};
use crate::config::error::ConfigError;
use crate::scene::scene::SceneBuilder;
use crate::datastructure::basic::BasicDataStructure;
use std::path::Path;
use crate::raytracer::basic::BasicRaytracer;
use crate::shader::mtlshader::MtlShader;
use crate::renderer::RendererBuilder;
use crate::util::camera::Camera;
use crate::datastructure::bvh::KDTreeDataStructure;
use crate::shader::mcshader::McShader;
use crate::shader::vmcshader::VMcShader;
use crate::datastructure::DataStructure;
use crate::shader::Shader;
use std::borrow::Borrow;
use crate::raytracer::RayTracer;

impl Config {
    pub fn run(self) -> Result<(), ConfigError>{
        let tobj = tobj::load_obj(self.general.scenename.as_ref())?;

        let scene = SceneBuilder::new()
            .texturepath(Path::new(&self.general.texturepath))
            .build_from_tobj(tobj)?;

        let datastructure: Box<dyn DataStructure> = match self.datastructure {
            DatastructureConfig::basic => Box::new(BasicDataStructure::new(&scene)),
            DatastructureConfig::kdtree => Box::new(KDTreeDataStructure::new(&scene)),
        };

        let shader: Box<dyn Shader> = match self.shader {
            ShaderConfig::mtlshader => Box::new(MtlShader),
            ShaderConfig::mcshader => Box::new(McShader),
            ShaderConfig::vmcshader {air_density, particle_reflectivity} => Box::new(VMcShader::new(air_density, particle_reflectivity)),

        };

        let tracer: Box<dyn RayTracer> = match self.raytracer {
            RaytracerConfig::Basic => Box::new(BasicRaytracer),
        };

        let renderer = RendererBuilder::new(datastructure.as_ref())
            .with_shader(shader.as_ref())
            .with_tracer(tracer.as_ref())
            .without_postprocessor();

        let camera = Camera::new(
            self.camera.position,
            self.camera.width,
            self.camera.height,
            self.camera.fov,
        );


        renderer.render(&camera)
            .to_bmp()
            .save(self.general.outputname)?;


        Ok(())
    }
}