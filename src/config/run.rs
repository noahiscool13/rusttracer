use crate::config::{Config, DatastructureConfig, ShaderConfig, RaytracerConfig};
use crate::config::error::ConfigError;
use crate::scene::scene::SceneBuilder;
use crate::datastructure::basic::BasicDataStructure;
use std::path::Path;
use crate::raytracer::basic::BasicRaytracer;
use crate::shader::mtlshader::MtlShader;
use crate::renderer::RendererBuilder;
use crate::util::camera::Camera;

impl Config {
    pub fn run(self) -> Result<(), ConfigError>{
        let tobj = tobj::load_obj(self.general.scenename.as_ref())?;

        let scene = SceneBuilder::new()
            .texturepath(Path::new(&self.general.texturepath))
            .build_from_tobj(tobj)?;


        let datastructure = match self.datastructure {
            DatastructureConfig::Basic => BasicDataStructure::new(&scene),
        };

        let shader = match self.shader {
            ShaderConfig::MtlShader => MtlShader,
        };

        let tracer = match self.raytracer {
            RaytracerConfig::Basic => BasicRaytracer,
        };

        let renderer = RendererBuilder::new(&datastructure)
            .with_shader(&shader)
            .with_tracer(&tracer)
            .without_postprocessor();

        let camera = Camera::new(
            self.camera.position,
            self.camera.width,
            self.camera.height,
            self.camera.fov,
        );

        let output = renderer.render(&camera)
            .to_bmp()
            .save(self.general.outputname)?;


        Ok(())
    }
}