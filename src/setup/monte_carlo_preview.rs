use crate::setup::Setup;
use std::process;
use crate::scene::scene::SceneBuilder;
use std::path::Path;
use crate::renderer::RendererBuilder;
use crate::datastructure::basic::BasicDataStructure;
use crate::shader::vmcshader::VMcShader;
use crate::raytracer::jmstrace::JMSTracer;
use crate::util::vector::Vector;
use crate::util::camera::Camera;
use crate::previewer::Previewer;
use crate::error::{RustTracerResult, RustTracerError};


pub struct MonteCarloPreview;

impl Setup for MonteCarloPreview {
    fn run(&self) -> RustTracerResult<()> {

        let tobj = tobj::load_obj("scenes/monte-carlo.obj".as_ref()).map_err(RustTracerError::TobjError)?;

        let scene = SceneBuilder::new()
            .texturepath(Path::new("scenes"))
            .build_from_tobj(tobj)
            .map_err(RustTracerError::SceneError)?;



        let renderer = RendererBuilder::new(&scene)
            .with_datastructure::<BasicDataStructure>()
            .with_shader(VMcShader)
            .with_tracer(JMSTracer)
            .without_postprocessor();

        let previewer = Previewer::new(&scene).map_err(RustTracerError::PreviewError)?;

        let camera = previewer.get_camera();

        renderer.render(&camera).to_bmp().save("render.bmp").expect("Couldn't save");

        Ok(())
    }
}