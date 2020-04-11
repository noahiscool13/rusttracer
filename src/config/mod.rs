use crate::config::corecount::ThreadCount;
use crate::util::vector::Vector;
use crate::config::error::ConfigError;
use std::path::Path;
use std::fs;
use serde::{Serialize, Deserialize};

pub mod error;
pub mod defaults;
pub mod corecount;
pub mod run;

#[derive(Serialize, Deserialize)]
// allow because names here are converted to yml keys which I want lowercase
#[allow(non_camel_case_types)]
pub struct Config {
    general: GeneralConfig,
    camera: CameraConfig,
    generator: GeneratorConfig,
    raytracer: RaytracerConfig,
    shader: ShaderConfig,
    datastructure: DatastructureConfig,
}

#[derive(Serialize, Deserialize)]
// allow because names here are converted to yml keys which I want lowercase
#[allow(non_camel_case_types)]
pub enum RaytracerConfig {
    /// Simple raytracing. Cast one ray per pixel
    basic,
    /// Use a multisampling raytracer. Samples every pixel n times.
    jmstracer {
        samples_per_pixel: usize
    },
    /// Use a multisampling raytracer that jitters (randomizes) the rays
    /// slightly. Samples every pixel n times.
    mstracer {
        samples_per_pixel: usize
    },
}

#[derive(Serialize, Deserialize)]
// allow because names here are converted to yml keys which I want lowercase
#[allow(non_camel_case_types)]
pub enum GeneratorConfig {
    /// Don't use any multithreading
    basic,

    /// Make use of the crossbeam library to spawn threads.
    /// This can have an advantage over rayon since there's no need for scheduling.
    crossbeam {
        /// The number of cores to use during the raytracing.
        threads: ThreadCount,
    },

    /// Make use of the rayon library by parallel-iterating over the pixels that have to be rendered.
    rayon {
        /// The number of cores to use during the raytracing.
        threads: ThreadCount,
    }
}

#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Very small float value.
    /// Two floats that are closer together than this value will be equal.
    epsilon: f64,

    /// Filename of the scene that will render
    scenename: String,

    /// Filename of the generated bitmap
    outputname: String,

    /// Path to search for texture files
    texturepath: String,
}

#[derive(Serialize, Deserialize)]
pub struct CameraConfig {

    /// The position of the camera in 3d space
    position: Vector,

    /// The width of the image to be generated
    width: usize,
    /// The height of the image to be generated
    height: usize,

    /// The field of view of the camera
    fov: f64,
}

#[derive(Serialize, Deserialize)]
// allow because names here are converted to yml keys which I want lowercase
#[allow(non_camel_case_types)]
pub enum ShaderConfig {
    /// Simple shader that shades based on the material of the triangle that was hit
    mtlshader,

    /// More advanced shader that uses monte carlo raytracing or pathtracing.
    /// (https://en.wikipedia.org/wiki/Path_tracing)
    mcshader,
    vmcshader {
        air_density: f64,
        particle_reflectivity: f64
    }
}

#[derive(Serialize, Deserialize)]
// allow because names here are converted to yml keys which I want lowercase
#[allow(non_camel_case_types)]
pub enum DatastructureConfig {
    /// Don't use any datastructure. Just iterate through the triangles of the scene.
    basic,
    /// Use a kdtree as a datastructure to speed up rendering of large scenes.
    kdtree
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: Default::default(),
            camera: Default::default(),

            generator: Default::default(),
            raytracer: Default::default(),
            shader: Default::default(),
            datastructure: Default::default(),
        }
    }
}


impl Config {
    pub fn dump(&self, filename: impl AsRef<Path>) -> Result<(), ConfigError> {
        let yamlstring = serde_yaml::to_string(self)?;

        fs::write(filename, yamlstring)?;

        Ok(())
    }

    pub fn load(filename: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let contents = fs::read(filename)?;

        Ok(serde_yaml::from_slice(&contents)?)
    }
}


