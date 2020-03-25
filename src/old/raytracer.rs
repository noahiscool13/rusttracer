use crate::color::Color;
use crate::ray::{Ray, IntersectReturnType};
use crate::triangle::Triangle;
use bmp::{Image, px, Pixel};
use crate::camera::Camera;
use crate::scene::Scene;
use std::f64;
use rayon::prelude::*;
use crate::shaders::{diffuse, specular};
use crate::vector::Vector;

pub struct RayTracer<'r> {
    buffer: Vec<Vec<Color>>,
    scene: Scene<'r>,
}

impl<'r> RayTracer<'r> {
    pub fn new(object: Scene<'r>) -> Self {
        Self {
            buffer: vec![],
            scene: object,
        }
    }

    pub fn load_scene(&mut self, object: Scene<'r>) {
        self.scene = object;
    }

    pub fn to_bmp(&self) -> Image {
        let height = self.buffer.len();
        let width = if height > 0 {
            self.buffer[0].len()
        } else { 0 };

        let mut img = Image::new(width as u32, height as u32);

        for (x, y) in img.coordinates() {
            let color = &self.buffer[y as usize][x as usize];
            img.set_pixel(x, y, px!(color.r, color.g, color.b));
        }

        img
    }

    pub fn presize(&mut self, width: usize, height: usize) {
        self.buffer.clear();
        self.buffer.reserve(height);
        for i in 0..height {
            self.buffer.push(Vec::new());
            self.buffer[i].resize_with(width, Default::default)
        }
    }

    pub fn trace_ray(&self, ray: &Ray) -> Color {
        let mut tm = f64::INFINITY;
        let mut um = 0f64;
        let mut vm = 0f64;
        let mut idm = 0;
        for tri in self.scene.triangles() {
            let intersection = ray.intersects(&tri);

            if let Some(IntersectReturnType { u, v, t, faceindex: id }) = intersection {
                if t < tm {
                    tm = t;
                    um = u;
                    vm = v;
                    idm = id;
                }
            }
        }
        if tm.is_infinite() {
            return Color { r: 0, g: 0, b: 0 };
        }

        let hit_pos = ray.origin + ray.direction*tm;
        let dif = diffuse(&self.scene.faces[idm],hit_pos,Vector::new(0f64,0.5f64,5f64));
        let spec = specular(&self.scene.faces[idm],hit_pos,Vector::new(0f64,0.5f64,5f64),ray.origin);
        let col =(255f64*(dif+spec)*0.5f64).floor() as u8;
        return Color{r: col, g:col, b: col};
//        return Color { r: (255f64 - (tm * 50f64)).floor() as u8, g: (um * 255f64).floor() as u8, b: (vm * 255f64).floor() as u8 };
    }

    pub fn trace_img(&mut self, camera: Camera) {
        let mut buffer = Vec::new();

        buffer.reserve(camera.height);
        for i in 0..camera.height {
            buffer.push(Vec::new());
            buffer[i].resize_with(camera.width, Default::default)
        }

        buffer.par_iter_mut().enumerate().for_each(|(y, row)| {
            for x in 0..camera.width {
                let ray = camera.generate_ray(x, y);
                let col = self.trace_ray(&ray);
                row[x] = col;
            }
        });

        self.buffer = buffer;
    }
}