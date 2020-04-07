use crate::datastructure::DataStructure;
use crate::shader::Shader;
use crate::util::ray::Ray;
use crate::util::vector::Vector;

pub struct TestShader {}

impl<'s> Shader<'s> for TestShader {
    fn shade(&self, ray: &Ray, datastructure: &'s dyn DataStructure<'s>) -> Vector {
        let intersection = if let Some(intersection) = datastructure.intersects(&ray) {
            intersection
        } else {
            return Vector::repeated(0f64);
        };

        // let dif = diffuse(&intersection.face, self.scene, hit_pos, Vector::new(0f64, 0.5f64, 5f64));
        // let spec = specular(&intersection.face, self.scene, Vector::new(0f64,0.5f64,5f64),ray.origin);
        // let col =(255f64*(dif+spec)*0.5f64).floor() as u8;
        return Vector::new(
            intersection.t * 30f64 / 255f64,
            intersection.uv.0 * 120f64 / 255f64,
            intersection.uv.1 * 120f64 / 255f64,
        );
    }
}
