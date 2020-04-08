use crate::config::corecount::CoreCount;
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
pub struct Config {
    general: GeneralConfig,
    camera: CameraConfig,
    shader: ShaderConfig,
    datastructure: DatastructureConfig,
    raytracer: RaytracerConfig
}

#[derive(Serialize, Deserialize)]
pub enum RaytracerConfig {
    Basic,
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

    /// The number of cores to use during the raytracing.
    cores: CoreCount,
}

#[derive(Serialize, Deserialize)]
pub struct CameraConfig {

    /// The position of the camera in 3d space
    position: Vector,

    /// The width of the generated image
    width: usize,
    /// The height of the generated image
    height: usize,

    /// The field of view of the camera
    fov: f64,
}

#[derive(Serialize, Deserialize)]
pub enum ShaderConfig {
    mtlshader,
    mcshader,
    vmcshader {
        air_density: f64,
        particle_reflectivity: f64
    }
}

#[derive(Serialize, Deserialize)]
pub enum DatastructureConfig {
    basic,
    kdtree
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: Default::default(),
            camera: Default::default(),

            datastructure: Default::default(),
            shader: Default::default(),
            raytracer: Default::default(),
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


