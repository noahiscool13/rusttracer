use crate::renderer::Renderer;
use std::process;
use crate::util::camera::Camera;
use crate::datastructure::precalculated::PrecalculatedDatastructure;
use crate::util::vector::Vector;
use crate::raytracer::jmstrace::JMSTracer;
use crate::shader::vmcshader::VMcShader;
use crate::scene::scene::SceneBuilder;
use std::path::Path;
use crate::shader::mcshader::McShader;
use crate::datastructure::basic::BasicDataStructure;

mod datastructure;
mod raytracer;
mod util;
mod renderer;
mod scene;
mod shader;

fn main() {

    let tobj = tobj::load_obj("scenes/monte-carlo.obj".as_ref()).unwrap_or_else(|err| {
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

    let renderer = Renderer::<BasicDataStructure, _, _>::new(&scene, JMSTracer, McShader);


    let camera = Camera::new(Vector::new(0., 1., 3.),  1000, 1000, 60f64);
    renderer.render(&camera).to_bmp().save("render.bmp").expect("Couldn't save");
}