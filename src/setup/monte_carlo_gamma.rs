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
use crate::postprocessors::gamma::Gamma;

pub struct MonteCarloGamma;

impl Setup for MonteCarloGamma {
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


        let renderer = RendererBuilder::new(&scene)
            .with_datastructure::<BasicDataStructure>()
            .with_shader(VMcShader)
            .with_tracer(JMSTracer)
            .with_postprocessor(&Gamma);

        let camera = Camera::new(Vector::new(0.5, 2.0, 3.),  600, 600, 60f64);
        renderer.render(&camera).to_bmp().save("render.bmp").expect("Couldn't save");
    }
}