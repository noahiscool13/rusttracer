use crate::setup::monte_carlo::MonteCarlo;
use crate::setup::Setup;

use log::info;
use log::LevelFilter;
use simple_logging;
use crate::setup::glowstone_gamma::GlowStoneGamma;
use crate::setup::house_cb::HouseCB;

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

    GlowStoneGamma.run()
}
