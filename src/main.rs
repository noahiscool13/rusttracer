use crate::setup::monte_carlo::MonteCarlo;
use crate::setup::Setup;

use simple_logging;
use log::LevelFilter;
use log::info;
use crate::setup::monte_carlo_gamma::MonteCarloGamma;

mod datastructure;
mod raytracer;
mod util;
mod renderer;
mod scene;
mod shader;
mod postprocessors;
mod setup;

fn main() {

    simple_logging::log_to_stderr(LevelFilter::Debug);
    info!("log :)");

    MonteCarlo.run()
}