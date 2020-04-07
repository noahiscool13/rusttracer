use crate::datastructure::basic::BasicDataStructure;
use crate::raytracer::crossbeamjmstrace::CrossbeamJMSTracer;
use crate::renderer::RendererBuilder;
use crate::scene::scene::SceneBuilder;
use crate::setup::Setup;
use crate::shader::mtlshader::MtlShader;
use crate::util::camera::Camera;
use crate::util::vector::Vector;
use std::path::Path;
use std::process;

pub struct HouseCB;

impl Setup for HouseCB {
    fn run(&self) {
        let tobj = tobj::load_obj("scenes/house.obj".as_ref()).unwrap_or_else(|err| {
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
            .with_shader(&MtlShader)
            .with_tracer(&CrossbeamJMSTracer)
            .without_postprocessor();

        let camera = Camera::new(Vector::new(-20., 10.0, 55.), 100, 100, 60f64);
        renderer
            .render(&camera)
            .to_bmp()
            .save("render.bmp")
            .expect("Couldn't save");
    }
}
