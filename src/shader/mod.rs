use crate::util::color::Color;
use crate::datastructure::intersection::Intersection;
use crate::scene::{Scene, Face};
use crate::util::vector::Vector;

pub mod testshader;
pub mod mtlshader;

// TODO: recursive shading
pub trait Shader<'s> {
    fn new(scene: &'s Scene) -> Self;
    fn shade(&self, intersection: &Intersection) -> Color;
    fn get_scene(&self) -> &'s Scene;

    fn ambient(&self, face: &Face) -> Vector {
        let scene = self.get_scene();
        Vector::from_arr(face.material(scene).ambient)
    }

    fn emittance(&self, face: &Face) -> Vector {
        let scene = self.get_scene();

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

    fn diffuse(&self, face: &Face, hit_pos: Vector, light_pos: Vector) -> Vector {
        let mut scene = self.get_scene();

        scene.textureatlas.get_texture(&face.material(scene).diffuse_texture);

        let light_dir = (light_pos - hit_pos).unit();
        light_dir.dot(face.normal(scene)).max(0.) * Vector::from_arr(face.material(scene).diffuse)
    }

    fn specular(&self, face: &Face, hit_pos: Vector, light_pos: Vector, cam_pos: Vector) -> Vector {
        let scene = self.get_scene();

        let light_dir = (light_pos - hit_pos).unit();
        let reflec = 2f64 * (face.normal(scene).dot(light_dir)) * face.normal(scene) - light_dir;
        let spec = 0f64.max((cam_pos - hit_pos).unit().dot(reflec));
        spec.powf(face.material(scene).shininess as f64) * Vector::from_arr(face.material(scene).specular)
    }
}