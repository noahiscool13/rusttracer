use crate::datastructure::DataStructure;
use crate::shader::shaders::{emittance, map_uv};
use crate::shader::Shader;
use crate::util::ray::Ray;
use crate::util::rng::get_rng;
use crate::util::vector::Vector;
use rand::Rng;
use std::f64;

#[derive(Debug)]
pub struct VMcShader {
    air_density: f64,
    particle_reflectivity: f64,
}

impl VMcShader {
    pub fn new(air_density: f64, particle_reflectivity: f64) -> Self{
        Self {
            air_density,
            particle_reflectivity,
        }
    }

    pub fn shade_internal<'a>(
        &self,
        ray: &Ray,
        depth: usize,
        datastructure: &'a (dyn DataStructure + 'a),
    ) -> Vector {
        let intersection = if let Some(intersection) = datastructure.intersects(&ray) {
            intersection
        } else {
            return if depth > 0 {
                let reflec_type = get_rng(|mut r| r.gen::<f64>());
                if self.particle_reflectivity > reflec_type {
                    let breakdist = -get_rng(|mut r| r.gen::<f64>()).ln() / self.air_density;
                    let hit_point = ray.origin + ray.direction * breakdist;
                    let scatter_ray = Ray::new(hit_point, Vector::point_on_sphere());
                    self.shade_internal(&scatter_ray, depth - 1, datastructure)
                } else {
                    Vector::repeated(0f64)
                }
            } else {
                Vector::repeated(0f64)
            }
        };

        let hit_pos = intersection.hit_pos();
        let dist = (ray.origin - hit_pos).length();
        let breakdist = -get_rng(|mut r| r.gen::<f64>()).ln() / self.air_density;

        if breakdist < dist {
            let reflec_type = get_rng(|mut r| r.gen::<f64>());
            if self.particle_reflectivity > reflec_type {
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
            let diffuse_max = intersection.triangle.material().diffuse.max_item();
            if diffuse_max > reflec_type {
                let fliped_normal = if intersection.triangle.normal().dot(ray.direction)<0.{
                    intersection.triangle.normal()
                } else {
                    intersection.triangle.normal()*-1.
                };
                let bounce_direction =
                    Vector::point_on_diffuse_hemisphere().rotated(fliped_normal);
                let bounce_ray = Ray::new(hit_pos, bounce_direction);
                let indirect_light = self.shade_internal(&bounce_ray, depth - 1, datastructure);
                let texture =
                    if let Some(texture) = intersection.triangle.mesh.material.diffuse_texture {
                        let coord = map_uv(&intersection);
                        texture.at(coord)
                    } else {
                        Vector::new(1., 1., 1.)
                    };
                indirect_light * intersection.triangle.material().diffuse / diffuse_max * texture
            } else {
                Vector::repeated(0f64)
            }
        } else {
            Vector::repeated(0f64)
        };

        indirect + part_emi
    }
}

impl Shader for VMcShader {
    fn shade<'s>(&self, ray: &Ray, datastructure: &'s (dyn DataStructure + 's)) -> Vector {
        self.shade_internal(ray, 6, datastructure)
    }
}
