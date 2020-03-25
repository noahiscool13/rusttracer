use crate::scene::Scene;
use crate::shader::Shader;
use crate::datastructure::intersection::Intersection;
use crate::util::color::Color;
use crate::util::vector::Vector;


pub struct MtlShader<'s> {
    scene: &'s Scene
}

impl<'s> Shader<'s> for MtlShader<'s> {
    fn new(scene: &'s Scene) -> Self {
        Self {
            scene
        }
    }

    fn get_scene(&self) -> &'s Scene {
        self.scene
    }

    fn shade(&self, intersection: &Intersection) -> Color {
        let pointlight = Vector::new(0f64, 0.2f64, 1f64);
        let brightness = Vector::repeated(1f64);

        let hit_pos = intersection.hit_pos();

        let part_amb = self.ambient(&intersection.face) * Vector::repeated(0.1);
        let part_emi = self.emittance(&intersection.face);
        let part_diff = self.diffuse(&intersection.face, hit_pos, pointlight) * brightness;
        let part_spec = self.specular(&intersection.face, hit_pos, pointlight, intersection.ray.origin) * brightness;


        let total = part_amb + part_emi + part_diff + part_spec;


        // let dif = diffuse(&intersection.face, self.scene, hit_pos, Vector::new(0f64, 0.5f64, 5f64));
        // let spec = specular(&intersection.face, self.scene, Vector::new(0f64,0.5f64,5f64),ray.origin);
        // let col =(255f64*(dif+spec)*0.5f64).floor() as u8;
        return total.into();
    }
}