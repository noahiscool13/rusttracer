use crate::setup::monte_carlo::MonteCarlo;
use crate::setup::Setup;

use simple_logging;
use log::LevelFilter;
use log::error;
use std::process;
use crate::setup::monte_carlo_preview::MonteCarloPreview;

mod datastructure;
mod raytracer;
mod util;
mod renderer;
mod scene;
mod shader;
mod postprocessors;
mod setup;
mod previewer;
mod error;

fn main() {

    simple_logging::log_to_stderr(LevelFilter::Debug);

    MonteCarloPreview.run().unwrap_or_else(|err| {
        error!("An error occured: {:?}", err);
        process::exit(1);
    });
}