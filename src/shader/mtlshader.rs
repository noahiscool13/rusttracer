use crate::scene::Scene;
use crate::shader::Shader;
use crate::datastructure::intersection::Intersection;
use crate::util::color::Color;
use crate::shader::shaders::{ambient, emittance, diffuse, specular};
use crate::util::vector::Vector;
use crate::datastructure::DataStructure;
use crate::util::ray::Ray;

pub struct MtlShader<'s> {
    scene: &'s Scene
}

impl<'s, DS: DataStructure<'s>> Shader<'s, DS> for MtlShader<'s> {
    fn new(scene: &'s Scene) -> Self {
        Self {
            scene
        }
    }

    fn shade(&self, ray:Ray, datastructure: &DS) -> Vector {

        let intersection = if let Some(intersection) = datastructure.intersects(&ray) {
            intersection
        } else {
            return Vector::repeated(0f64)
        };

        let pointlight = Vector::new(0f64, 0.2f64, 1f64);
        let brightness = Vector::repeated(1f64);

        let hit_pos = intersection.hit_pos();

        let part_amb = ambient(&intersection.face, self.scene) * Vector::repeated(0.1);
        let part_emi = emittance(&intersection.face, self.scene);
        let part_diff = diffuse(&intersection.face, self.scene, hit_pos, pointlight) * brightness;
        let part_spec = specular(&intersection.face, self.scene, hit_pos, pointlight, intersection.ray.origin) * brightness;


        let total = part_amb + part_emi + part_diff + part_spec;

        return total;
    }
}