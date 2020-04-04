

mod datastructure;
mod postprocessors;
mod raytracer;
mod renderer;
mod scene;
mod setup;
mod shader;
mod util;

#[allow(unused_imports)]
mod child {
    use log::LevelFilter;
    use crate::setup::monte_carlo_bvh::MonteCarloBVH;
    use crate::setup::Setup;
    use crate::setup::house_bvh::HouseBVH;

    pub fn main() {
        simple_logging::log_to_stderr(LevelFilter::Debug);

        HouseBVH.run()
    }
}

fn main() {
    child::main()
}
