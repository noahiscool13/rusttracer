use crate::util::vector::Vector;
use crate::scene::{Face, Scene};

pub fn ambient(face: &Face, scene: &Scene) -> Vector {
     Vector::from_arr(face.material(scene).ambient)
 }

 pub fn emittance(face: &Face, scene: &Scene) -> Vector {
     let material = face.material(scene);
     let default = "0.0 0.0 0.0".into();
     let strval = material.unknown_param.get("Ke").unwrap_or(&default);

     let res: Vec<f64> = strval.split(" ")
         .map(|i| i.parse())
         .collect::<Result<Vec<f64>, _>>()
         .unwrap_or(vec![0., 0., 0.]);

     if res.len() != 3 {
         Vector::new(0., 0., 0.)
     } else {
         Vector::new(res[0], res[1], res[2])
     }

 }

 pub fn diffuse(face: &Face, scene: &Scene, hit_pos: Vector, light_pos: Vector) -> Vector{
     let light_dir = (light_pos-hit_pos).unit();
     light_dir.dot(face.normal(scene)).max(0.) * Vector::from_arr(face.material(scene).diffuse)
 }


 pub fn specular(face: &Face, scene: &Scene, hit_pos: Vector, light_pos: Vector, cam_pos: Vector) -> Vector{
     let light_dir = (light_pos-hit_pos).unit();
     let reflec = 2f64 * (face.normal(scene).dot(light_dir)) * face.normal(scene) - light_dir;
     let spec = 0f64.max((cam_pos - hit_pos).unit().dot(reflec));
     spec.powf(face.material(scene).shininess as f64) * Vector::from_arr(face.material(scene).specular)
 }

