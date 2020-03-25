use crate::datastructure::DataStructure;
use crate::util::ray::Ray;
use crate::datastructure::intersection::Intersection;
use crate::util::triangle::Triangle;
use std::f64;
use crate::scene::Scene;

const EPSILON: f64 = 0.00001;

pub struct PrecalculatedDatastructure<'d> {
    data: &'d Scene,
    triangles: Vec<Triangle>,
}

impl<'d> PrecalculatedDatastructure<'d> {

    fn intersects_triangle<'a>(&self, ray: &'a Ray, triangle: &Triangle) -> Option<Intersection<'a>> {
        let edge1 = triangle.b - triangle.a;
        let edge2 = triangle.c - triangle.a;

        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if -EPSILON < a && a < EPSILON {
            return None;
        }

        let f = 1f64 / a;

        let s = ray.origin - triangle.a;
        let u = f * s.dot(h);

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);

        if u < 0f64 || u > 1f64 {
            return None;
        }

        if v < 0f64 || u + v > 1f64 {
            return None;
        }

        let t = f * edge2.dot(q);
        if t < EPSILON {
            return None;
        }

        Some(Intersection { uv: (u, v), t, ray, face: triangle.face})
    }
}

impl<'d> DataStructure<'d> for PrecalculatedDatastructure<'d> {
    fn new(scene: &'d Scene) -> Self {
        Self {data: scene, triangles: scene.triangles().collect()}
    }

    fn intersects<'a>(&self, ray: &'a Ray) -> Option<Intersection<'a>> {
        let mut min = None;

        for triangle in &self.triangles {
            if let Some(intersection) = self.intersects_triangle(ray, triangle) {
                min = match min {
                    None => Some(intersection),
                    Some(i) if intersection.t < i.t => Some(intersection),
                    _ => min
                };
            }
        }

        return min;
        // let hit_pos = ray.origin + ray.direction*tm;
        // let dif = diffuse(&self.scene.faces[idm],hit_pos,Vector::new(0f64,0.5f64,5f64));
        // let spec = specular(&self.scene.faces[idm],hit_pos,Vector::new(0f64,0.5f64,5f64),ray.origin);
        // let col =(255f64*(dif+spec)*0.5f64).floor() as u8;
        // return Color{r: col, g:col, b: col};
    }
}
