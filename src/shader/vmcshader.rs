use crate::shader::Shader;
use crate::datastructure::intersection::Intersection;
use crate::util::color::Color;
use crate::shader::shaders::{ambient, emittance, diffuse, specular};
use crate::util::vector::Vector;
use crate::util::ray::Ray;
use crate::datastructure::DataStructure;
use rand::{thread_rng, Rng};
use crate::scene::scene::Scene;

const AIR_DENS : f64 = 0.3f64;

pub struct VMcShader<'s> {
    scene: &'s Scene<'s>
}

impl <'s> VMcShader<'s> {
    pub fn shade_internal<'a, DS : DataStructure<'a>>(&self, ray : Ray, depth:usize, datastructure: &DS) -> Vector{
//        let pointlight = Vector::new(0f64, 0.2f64, 1f64);
//        let brightness = Vector::repeated(0f64);
        let mut rng = thread_rng();

        let intersection = if let Some(intersection) = datastructure.intersects(&ray) {
            intersection
        } else {
            if depth > 0 {
                let breakdist = -(rng.gen::<f64>()).ln() / AIR_DENS;
                let hit_point = ray.origin + ray.direction * breakdist;
                let scatter_ray = Ray::new(hit_point, Vector::point_on_sphere());
                return self.shade_internal(scatter_ray, depth - 1, datastructure);
            } else {
                return Vector::repeated(0f64);
            }
        };
//
        let hit_pos = intersection.hit_pos();

        let dist = (ray.origin-hit_pos).length();

        let breakdist = -(rng.gen::<f64>()).ln()/AIR_DENS;

        if breakdist<dist {
            let hit_point = ray.origin+ray.direction*breakdist;
            let scatter_ray = Ray::new(hit_point, Vector::point_on_sphere());
            if depth > 0 {
            return self.shade_internal(scatter_ray,depth-1,datastructure);
            } else {
                return Vector::repeated(0f64);
            }
        }
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
                let indirect_light = self.shade_internal(bounce_ray,depth-1,datastructure);
                indirect_light * diffuse(&intersection, hit_pos, hit_pos+bounce_direction)
            } else {
                Vector::repeated(0f64)
            };

        let total =  indirect + part_emi;

        return total.into();
    }
}

impl<'s, DS: DataStructure<'s>> Shader<'s, DS> for VMcShader<'s> {
    fn new(scene: &'s Scene) -> Self {
        Self {
            scene
        }
    }

    fn shade(&self, ray: Ray, datastructure: &DS) -> Vector {
        self.shade_internal(ray,6, datastructure)
    }
}