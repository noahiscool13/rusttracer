use crate::setup::monte_carlo::MonteCarlo;
use crate::setup::Setup;

use crate::setup::monte_carlo_crossbeam::MonteCarloCrossbeam;
use crate::setup::monte_carlo_bvh::MonteCarloBVH;
use log::info;
use log::LevelFilter;
use simple_logging;

mod datastructure;
mod postprocessors;
mod raytracer;
mod renderer;
mod scene;
mod setup;
mod shader;
mod util;

fn main() {
    simple_logging::log_to_stderr(LevelFilter::Debug);
    info!("log :)");

    MonteCarloBVH.run()
}
