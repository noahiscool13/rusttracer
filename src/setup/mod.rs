///! This module defines a render setup. It basically sets up a predefined pipeline of postprocessors, cameras, shaders and renderers.
pub mod glowstone_gamma;
pub mod mc_cornell_box_gamma;
pub mod monte_carlo;
pub mod monte_carlo_bvh;
pub mod monte_carlo_crossbeam;
pub mod monte_carlo_gamma;

pub trait Setup {
    fn run(&self);
}
