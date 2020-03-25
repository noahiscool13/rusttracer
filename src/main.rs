use crate::renderer::Renderer;
use std::process;
use crate::util::camera::Camera;
use crate::scene::Scene;
use crate::raytracer::rayon::RayonRaytracer;
use crate::datastructure::precalculated::PrecalculatedDatastructure;
use crate::util::vector::Vector;
use crate::shader::mtlshader::MtlShader;

mod datastructure;
mod raytracer;
mod util;
mod renderer;
mod scene;
mod shader;

fn main() {
    let scene = Scene::new_tobj(tobj::load_obj("scenes/glowstone.obj".as_ref()).unwrap_or_else(|err| {
        eprintln!("Couldn't open file: {}", err);
        process::exit(1);
    })).unwrap_or_else(|err| {
        eprintln!("Texture error: {:?}", err);
        process::exit(1);
    });

    let renderer: Renderer<PrecalculatedDatastructure, RayonRaytracer, MtlShader> = Renderer::new(&scene);


    let camera = Camera::new(Vector::new(0f64, 1f64, 3f64), 1000, 1000, 60f64);
    renderer.render(&camera).to_bmp().save("render.bmp").expect("Couldn't save");
}