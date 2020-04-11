use crate::config::{Config, DatastructureConfig, ShaderConfig, RaytracerConfig, GeneratorConfig};
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
use crate::generator::Generator;
use crate::generator::basic::BasicGenerator;
use crate::raytracer::jmstracer::JMSTracer;
use crate::raytracer::mstracer::MSTracer;
use crate::generator::crossbeam::CrossbeamGenerator;
use crate::generator::rayon::RayonGenerator;

impl Config {
    pub fn run(self) -> Result<(), ConfigError>{
        let tobj = tobj::load_obj(self.general.scenename.as_ref())?;

        let scene = SceneBuilder::new()
            .texturepath(Path::new(&self.general.texturepath))
            .build_from_tobj(tobj)?;


        let generator: Box<dyn Generator> = match self.generator {
            GeneratorConfig::basic => Box::new(BasicGenerator),
            GeneratorConfig::crossbeam { threads } => Box::new(CrossbeamGenerator::new(threads.get_cores())),
            GeneratorConfig::rayon { threads } => Box::new(RayonGenerator::new(threads.get_cores()))
        };

        let raytracer: Box<dyn RayTracer> = match self.raytracer {
            RaytracerConfig::basic => Box::new(BasicRaytracer),
            RaytracerConfig::jmstracer { samples_per_pixel } => Box::new(JMSTracer::new(samples_per_pixel)),
            RaytracerConfig::mstracer { samples_per_pixel } => Box::new(MSTracer::new(samples_per_pixel)),
        };

        let shader: Box<dyn Shader> = match self.shader {
            ShaderConfig::mtlshader => Box::new(MtlShader),
            ShaderConfig::mcshader => Box::new(McShader),
            ShaderConfig::vmcshader {air_density, particle_reflectivity} => Box::new(VMcShader::new(air_density, particle_reflectivity)),
        };

        let datastructure: Box<dyn DataStructure> = match self.datastructure {
            DatastructureConfig::basic => Box::new(BasicDataStructure::new(&scene)),
            DatastructureConfig::kdtree => Box::new(KDTreeDataStructure::new(&scene)),
        };

        let renderer = RendererBuilder::new(generator.as_ref())
            .with_raytracer(raytracer.as_ref())
            .with_shader(shader.as_ref())
            .with_datastructure(datastructure.as_ref())
            .without_postprocessor();

        let camera = Camera::new(
            self.camera.position,
            self.camera.width,
            self.camera.height,
            self.camera.fov,
        );

        dbg!(&renderer);

        renderer.render(&camera)
            .to_bmp()
            .save(self.general.outputname)?;


        Ok(())
    }
}