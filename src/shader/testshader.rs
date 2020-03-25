use crate::shader::Shader;
use crate::datastructure::intersection::Intersection;
use crate::util::color::Color;
use crate::scene::Scene;

pub struct TestShader<'s> {
    scene: &'s Scene
}

impl<'s> Shader<'s> for TestShader<'s> {
    fn new(scene: &'s Scene) -> Self {
        Self {
            scene
        }
    }

    fn shade(&self, intersection: &Intersection) -> Color {

        // let dif = diffuse(&intersection.face, self.scene, hit_pos, Vector::new(0f64, 0.5f64, 5f64));
        // let spec = specular(&intersection.face, self.scene, Vector::new(0f64,0.5f64,5f64),ray.origin);
        // let col =(255f64*(dif+spec)*0.5f64).floor() as u8;
        return Color{r: (intersection.t * 30f64) as u8, g:(intersection.uv.0 * 120f64) as u8, b: (intersection.uv.1 * 120f64) as u8};
    }

    fn get_scene(&self) -> &'s Scene {
        self.scene
    }
}