use crate::util::vector::Vector;
use crate::config::{CameraConfig, DatastructureConfig, GeneralConfig, ShaderConfig, RaytracerConfig, GeneratorConfig};
use crate::config::corecount::ThreadCount;

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            epsilon: 0.00001,
            scenename: "test".to_string(),
            outputname: "render.bmp".to_string(),
            texturepath: "scenes".to_string(),
        }
    }
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            position: Vector::default(),
            width: 1000,
            height: 1000,
            fov: 60.,
        }
    }
}

impl Default for DatastructureConfig {
    fn default() -> Self {
        DatastructureConfig::kdtree
    }
}

impl Default for ShaderConfig {
    fn default() -> Self {
        ShaderConfig::vmcshader {
            air_density: 0.3,
            particle_reflectivity: 0.4,
        }
    }
}

impl Default for RaytracerConfig {
    fn default() -> Self {
        RaytracerConfig::basic
    }
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        GeneratorConfig::basic
    }
}