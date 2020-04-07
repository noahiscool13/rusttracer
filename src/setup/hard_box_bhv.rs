use crate::datastructure::bvh::KDTreeDataStructure;
use crate::raytracer::jmstrace::JMSTracer;
use crate::renderer::RendererBuilder;
use crate::scene::scene::SceneBuilder;
use crate::setup::Setup;
use crate::shader::vmcshader::VMcShader;
use crate::util::camera::Camera;
use crate::util::vector::Vector;
use std::path::Path;
use std::process;
use crate::shader::mtlshader::MtlShader;

pub struct HardBoxBHV;

impl Setup for HardBoxBHV {
    fn run(&self) {
        let tobj = tobj::load_obj("scenes/hard-box.obj".as_ref()).unwrap_or_else(|err| {
            eprintln!("Couldn't open obj file: {}", err);
            process::exit(1);
        });

        let scene = SceneBuilder::new()
            .texturepath(Path::new("scenes"))
            .build_from_tobj(tobj)
            .unwrap_or_else(|err| {
                eprintln!("Couldn't create scene: {:?}", err);
                process::exit(1);
            });

        let renderer = RendererBuilder::new(&scene)
            .with_datastructure::<KDTreeDataStructure>()
            .with_shader(VMcShader)
            .with_tracer(JMSTracer)
            .without_postprocessor();

        let camera = Camera::new(Vector::new(3., 1.0, 0.), 1000, 1000, 60f64);
        renderer
            .render(&camera)
            .to_bmp()
            .save("render.bmp")
            .expect("Couldn't save");
    }
}
