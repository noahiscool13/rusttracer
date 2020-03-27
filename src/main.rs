use crate::setup::monte_carlo::MonteCarlo;
use crate::setup::Setup;

mod datastructure;
mod raytracer;
mod util;
mod renderer;
mod scene;
mod shader;
mod postprocessors;
mod setup;

fn main() {
    MonteCarlo.run()
}