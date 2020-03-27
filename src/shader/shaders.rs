use crate::util::vector::Vector;
use crate::datastructure::intersection::Intersection;
use crate::scene::texturecoordinate::TextureCoordinate;
use log::error;

pub fn ambient(intersection: &Intersection) -> Vector {
    let texture = if let Some(texture) = intersection.triangle.mesh.material.ambient_texture {
        let coord = map_uv(intersection);

        texture.at(coord)
    } else {
        Vector::new(1., 1., 1.)
    };

    intersection.triangle.material().ambient * texture
}

pub fn emittance(intersection: &Intersection) -> Vector {
    let texture = if let Some(texture) = intersection.triangle.mesh.material.emittance_texture {
        let coord = map_uv(intersection);

        texture.at(coord)
    } else {
        Vector::new(1., 1., 1.)
    };

    intersection.triangle.material().emittance * texture
}

pub fn map_uv(intersection: &Intersection) -> TextureCoordinate{
    let texa = intersection.triangle.texture_a();
    let texb = intersection.triangle.texture_b();
    let texc = intersection.triangle.texture_c();

    let e1 = texc-texa;
    let e2 = texb-texa;

//    error!("e1: {:?}, e2: {:?}", e1, e2);

    texa.to_owned() + (e1 * intersection.uv.1) + (e2 * intersection.uv.0)
}

pub fn diffuse(intersection: &Intersection, hit_pos: Vector, light_pos: Vector) -> Vector {
    let triangle = intersection.triangle;

    let texture = if let Some(texture) = intersection.triangle.mesh.material.diffuse_texture {
        let coord = map_uv(intersection);

        texture.at(coord)
    } else {
        Vector::new(1., 1., 1.)
    };

    let light_dir = (light_pos - hit_pos).unit();
    light_dir.dot(triangle.normal()).max(0.) * triangle.material().diffuse * texture
}

pub fn specular(intersection: &Intersection, hit_pos: Vector, light_pos: Vector, cam_pos: Vector) -> Vector {
    let texture = if let Some(texture) = intersection.triangle.mesh.material.specular_texture {
        let coord = map_uv(intersection);

        texture.at(coord)
    } else {
        Vector::new(1., 1., 1.)
    };

    let triangle = intersection.triangle;

    let light_dir = (light_pos - hit_pos).unit();
    let reflec = 2f64 * (triangle.normal().dot(light_dir)) * triangle.normal() - light_dir;
    let spec = 0f64.max((cam_pos - hit_pos).unit().dot(reflec));

    spec.powf(triangle.material().shininess) * triangle.material().specular * texture
}