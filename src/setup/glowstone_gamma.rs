use crate::datastructure::basic::BasicDataStructure;
use crate::postprocessors::gamma::Gamma;
use crate::raytracer::jmstrace::JMSTracer;
use crate::renderer::RendererBuilder;
use crate::scene::scene::SceneBuilder;
use crate::setup::Setup;
use crate::shader::vmcshader::VMcShader;
use crate::util::camera::Camera;
use crate::util::vector::Vector;
use std::path::Path;
use std::process;

pub struct GlowStoneGamma;

impl Setup for GlowStoneGamma {
    fn run(&self) {
        let tobj = tobj::load_obj("scenes/glowstone.obj".as_ref()).unwrap_or_else(|err| {
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

        let ds = BasicDataStructure::new(&scene);

        let renderer = RendererBuilder::new(&ds)
            .with_shader(&VMcShader)
            .with_tracer(&JMSTracer)
            .with_postprocessor(&Gamma);

        let camera = Camera::new(Vector::new(0.5, 2.2, 3.), 1000, 1000, 60f64);
        renderer
            .render(&camera)
            .to_bmp()
            .save("render.bmp")
            .expect("Couldn't save");
    }
}
