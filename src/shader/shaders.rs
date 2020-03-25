use crate::util::vector::Vector;
use crate::scene::{Face, Scene};

// pub fn ambient(face: &Face, scene: &Scene, scene_light: Vector) -> f64 {
//     Vector::from_arr(face.material(scene).ambient) * scene_light
// }
//
// // pub fn emittance(face: &Face, scene: &Scene) ->f64 {
// //     face.material(scene).Ke
// // }
//
// pub fn diffuse(face: &Face, scene: &Scene, hit_pos: Vector, light_pos: Vector) -> f64{
//     let light_dir = (light_pos-hit_pos).unit();
//     (0f64.max(light_dir.dot(face.normal(scene)))) * face.material(scene).diffuse
// }
//
// pub fn specular(face: &Face, scene: &Scene, hit_pos: Vector, light_pos: Vector, cam_pos: Vector) -> f64{
//     let light_dir = (light_pos-hit_pos).unit();
//     let reflec = (2f64 * (face.normal(scene).dot(light_dir)) * face.normal(scene) - light_dir);
//     let spec = 0f64.max((cam_pos - hit_pos).unit().dot(reflec));
//     spec.powf(face.material(scene).shininess as f64) * face.material(scene).specular
// }
