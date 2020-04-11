
use log::info;
use log::LevelFilter;
use simple_logging;
use crate::config::Config;

mod config;
mod datastructure;
mod postprocessors;
mod raytracer;
mod renderer;
mod scene;
// mod setup;
mod shader;
mod util;
mod generator;

fn main() {
    simple_logging::log_to_stderr(LevelFilter::Debug);

    // Config::default().dump("config.yml")
    //     .unwrap();

    Config::load("configurations/monte-carlo.yml")
        .unwrap()
        .run()
        .unwrap();


    // GlowStoneGamma.run()
}
