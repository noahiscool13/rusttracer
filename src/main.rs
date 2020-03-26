use crate::renderer::Renderer;
use std::process;
use crate::util::camera::Camera;
use crate::raytracer::rayon::RayonRaytracer;
use crate::datastructure::precalculated::PrecalculatedDatastructure;
use crate::util::vector::Vector;
use crate::shader::mtlshader::MtlShader;
use crate::shader::mcshader::McShader;
use crate::raytracer::mstrace::MSTracer;
use crate::raytracer::jmstrace::JMSTracer;
use crate::scene::scene::SceneBuilder;
use std::path::Path;

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

    let renderer: Renderer<PrecalculatedDatastructure, JMSTracer, McShader> = Renderer::new(&scene);


    let camera = Camera::new(Vector::new(0f64, 1f64, 3f64), 1000, 1000, 60f64);
    renderer.render(&camera).to_bmp().save("render.bmp").expect("Couldn't save");
}