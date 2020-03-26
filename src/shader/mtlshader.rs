use crate::shader::Shader;
use crate::datastructure::intersection::Intersection;
use crate::util::color::Color;
use crate::util::vector::Vector;
use crate::scene::scene::Scene;
use crate::shader::shaders::{ambient, emittance, diffuse, specular};
use crate::datastructure::DataStructure;

pub struct MtlShader<'s> {
    scene: &'s Scene<'s>
}

impl<'s, DS: DataStructure<'s>> Shader<'s, DS> for MtlShader<'s> {
    fn new(scene: &'s Scene<'s>) -> Self {
        Self {
            scene
        }
    }

    fn shade(&self, intersection: &Intersection, _: &DS) -> Vector {
        let pointlight = Vector::new(0f64, 0.2f64, 1f64);
        let brightness = Vector::repeated(1f64);

        let hit_pos = intersection.hit_pos();

        let part_amb = ambient(&intersection) * Vector::repeated(0.1);
        let part_emi = emittance(&intersection);
        let part_diff = diffuse(&intersection, hit_pos, pointlight) * brightness;
        let part_spec = specular(&intersection, hit_pos, pointlight, intersection.ray.origin) * brightness;


        let total = part_amb + part_emi + part_diff + part_spec;

        return total;
    }
}