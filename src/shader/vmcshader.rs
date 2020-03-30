use crate::datastructure::DataStructure;
use crate::shader::shaders::{diffuse, emittance};
use crate::shader::Shader;
use crate::util::ray::Ray;
use crate::util::vector::Vector;
use std::f64;
use crate::util::rng::get_rng;
use rand::Rng;

const AIR_DENS: f64 = 0.05f64;
const PARTICLE_REFLECT: f64 = 0.2;

pub struct VMcShader;

impl VMcShader {
    pub fn shade_internal<'a, DS: DataStructure<'a>>(
        &self,
        ray: &Ray,
        depth: usize,
        datastructure: &DS,
    ) -> Vector {
        let intersection = if let Some(intersection) = datastructure.intersects(&ray) {
            intersection
        } else {
            if depth > 0 {
                let reflec_type = get_rng(|mut r| r.gen::<f64>());
                if PARTICLE_REFLECT>reflec_type {
                    let breakdist = -get_rng(|mut r| r.gen::<f64>()).ln() / AIR_DENS;
                    let hit_point = ray.origin + ray.direction * breakdist;
                    let scatter_ray = Ray::new(hit_point, Vector::point_on_sphere());
                    return self.shade_internal(&scatter_ray, depth - 1, datastructure);
                } else {
                    return Vector::repeated(0f64);
                }
            } else {
                return Vector::repeated(0f64);
            }
        };
        //
        let hit_pos = intersection.hit_pos();

        let dist = (ray.origin - hit_pos).length();

        let breakdist = -get_rng(|mut r| r.gen::<f64>()).ln() / AIR_DENS;

        if breakdist < dist {
            let reflec_type = get_rng(|mut r| r.gen::<f64>());
            if PARTICLE_REFLECT>reflec_type {
                let hit_point = ray.origin + ray.direction * breakdist;
                let scatter_ray = Ray::new(hit_point, Vector::point_on_sphere());
                if depth > 0 {
                    return self.shade_internal(&scatter_ray, depth - 1, datastructure);
                } else {
                    return Vector::repeated(0f64);
                }
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

        let indirect = if depth > 0 {
            let reflec_type = get_rng(|mut r| r.gen::<f64>());
            let diffuse_max = intersection.triangle.material().diffuse.max();
            if (diffuse_max>reflec_type) {
                let bounce_direction =
                    Vector::point_on_hemisphere().rotated(intersection.triangle.normal());
                let bounce_ray = Ray::new(hit_pos, bounce_direction);
                let indirect_light = self.shade_internal(&bounce_ray, depth - 1, datastructure);
                indirect_light * diffuse(&intersection, hit_pos, hit_pos + bounce_direction)/diffuse_max
            } else {
                Vector::repeated(0f64)
            }
        } else {
            Vector::repeated(0f64)
        };
        let total = indirect * 2. + part_emi;

        return total.into();
    }
}

impl<'s, DS: DataStructure<'s>> Shader<'s, DS> for VMcShader {
    fn shade(&self, ray: &Ray, datastructure: &DS) -> Vector {
        self.shade_internal(ray, 6, datastructure)
    }
}
