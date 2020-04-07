use crate::util::vector::Vector;
use crate::config::{CameraConfig, DatastructureConfig, GeneralConfig, ShaderConfig, RaytracerConfig};
use crate::config::corecount::CoreCount;

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            epsilon: 0.00001,
            scenename: "test".to_string(),
            outputname: "render.bmp".to_string(),
            texturepath: "scenes".to_string(),
            cores: Default::default(),
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
        DatastructureConfig::Basic
    }
}

impl Default for ShaderConfig {
    fn default() -> Self {
        ShaderConfig::MtlShader
    }
}

impl Default for RaytracerConfig {
    fn default() -> Self {
        RaytracerConfig::Basic
    }
}