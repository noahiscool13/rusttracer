use crate::shader::Shader;
use crate::datastructure::intersection::Intersection;
use crate::util::color::Color;
use crate::shader::shaders::{ambient, emittance, diffuse, specular};
use crate::util::vector::Vector;
use crate::util::ray::Ray;
use crate::datastructure::DataStructure;
use crate::scene::scene::Scene;

pub struct McShader<'s> {
    scene: &'s Scene<'s>
}

impl <'s> McShader<'s> {
    pub fn shade_internal<'a, DS : DataStructure<'a>>(&self, intersection: &Intersection, depth:usize, datastructure: &DS) -> Vector{
//        let pointlight = Vector::new(0f64, 0.2f64, 1f64);
//        let brightness = Vector::repeated(0f64);
//
        let hit_pos = intersection.hit_pos();
//
//        let part_amb = ambient(&intersection.face, self.scene) * Vector::repeated(0.1);
        let part_emi = emittance(&intersection);
//        let part_diff = diffuse(&intersection.face, self.scene, hit_pos, pointlight) * brightness;
//        let part_spec = specular(&intersection.face, self.scene, hit_pos, pointlight, intersection.ray.origin) * brightness;
//
//
//        let direct = part_amb + part_emi + part_diff + part_spec;

        let indirect =
            if depth > 0 {
                let bounce_direction = Vector::point_on_hemisphere().rotated(intersection.triangle.normal());
                let bounce_ray = Ray::new(hit_pos,bounce_direction);
                if let Some(bounce_intersection) = datastructure.intersects(&bounce_ray) {
                    let indirect_light = self.shade_internal(&bounce_intersection,depth-1,datastructure);
                    indirect_light * diffuse(&intersection, hit_pos, bounce_intersection.hit_pos())
                } else {
                    Vector::repeated(0f64)
                }
            } else {
                Vector::repeated(0f64)
            };

        let total =  indirect + part_emi;

        return total.into();
    }
}

impl<'s, DS: DataStructure<'s>> Shader<'s, DS> for McShader<'s> {
    fn new(scene: &'s Scene) -> Self {
        Self {
            scene
        }
    }

    fn shade(&self, intersection: &Intersection, datastructure: &DS) -> Vector {
        self.shade_internal(intersection,4, datastructure)
    }
}