use crate::vector::Vector;

pub enum IlluminationModel {
    ColorOnAmbientOff = 0,
    ColorOnAmbientOn = 1,
    HighlightOn = 2,
    ReflectionOnRaytraceOn = 3,
    TransparencyGlassOnRaytraceOn = 4,
    ReflectionFresnelOnRaytraceOn = 5,
    // Transparency
}

impl Default for IlluminationModel {
    fn default() -> Self {
        Self::ColorOnAmbientOff
    }
}

#[derive(Default)]
pub struct Material {
    // Specular
    pub Ks: Vector,
    // Diffuse
    pub Kd: Vector,
    // Ambient
    pub Ka: Vector,

    // Emittance
    pub Ke: Vector,

    // reflectivity
    pub Ns: f64,

    // Transparency
    pub Tr: f64,

    pub illum: IlluminationModel
}