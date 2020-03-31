use crate::datastructure::DataStructure;
use crate::shader::shaders::{ambient, diffuse, emittance, specular};
use crate::shader::Shader;
use crate::util::ray::Ray;
use crate::util::vector::Vector;

pub struct MtlShader;

impl<'s, DS: DataStructure<'s>> Shader<'s, DS> for MtlShader {
    fn shade(&self, ray: &Ray, datastructure: &DS) -> Vector {
        let intersection = if let Some(intersection) = datastructure.intersects(&ray) {
            intersection

        } else {
            return Vector::repeated(0f64);
        };

        let pointlight = Vector::new(0f64, 50f64, 50f64);
        let brightness = Vector::repeated(1f64);

        let hit_pos = intersection.hit_pos();

        let part_amb = ambient(&intersection) * Vector::repeated(0.1);
        let part_emi = emittance(&intersection);
        let part_diff = diffuse(&intersection, hit_pos, pointlight) * brightness;
        let part_spec =
            specular(&intersection, hit_pos, pointlight, intersection.ray.origin) * brightness;

        let total = part_amb + part_emi + part_diff + part_spec;

        return total;
    }
}
