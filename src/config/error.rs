use std::io;
use crate::scene::error::SceneError;

#[derive(Debug)]
pub enum ConfigError {
    YamlError(serde_yaml::Error),
    IoError(io::Error),
    TobjLoadError(tobj::LoadError),
    SceneError(SceneError),
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(e: serde_yaml::Error) -> Self {
        ConfigError::YamlError(e)
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::IoError(e)
    }
}

impl From<tobj::LoadError> for ConfigError {
    fn from(e: tobj::LoadError) -> Self {
        ConfigError::TobjLoadError(e)
    }
}

impl From<SceneError> for ConfigError {
    fn from(e: SceneError) -> Self {
        ConfigError::SceneError(e)
    }
}