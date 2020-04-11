
use crate::config::Config;
use log::info;
use log::LevelFilter;
use simple_logging;

mod config;
mod datastructure;
mod postprocessors;
mod raytracer;
mod renderer;
mod scene;
mod generator;
mod shader;
mod util;

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
