use crate::scene::material::Material;
use crate::scene::scene::Mesh;
use crate::scene::texturecoordinate::TextureCoordinate;
use crate::util::vector::Vector;

#[derive(Debug, Clone)]
pub struct Triangle<'t> {
    pub(super) a: usize,
    pub(super) b: usize,
    pub(super) c: usize,

    pub mesh: &'t Mesh<'t>,
}

impl<'t> Triangle<'t> {
    #[inline]
    pub fn a(&self) -> Vector {
        self.mesh.vertices[self.a]
    }

    #[inline]
    pub fn b(&self) -> Vector {
        self.mesh.vertices[self.b]
    }

    #[inline]
    pub fn c(&self) -> Vector {
        self.mesh.vertices[self.c]
    }

    #[inline]
    pub fn material(&self) -> &'t Material {
        self.mesh.material
    }

    pub fn normal(&self) -> Vector {
        // TODO: depends on illum model

        (self.c() - self.a()).cross(self.c() - self.b()).unit()
    }

    #[inline]
    pub fn texture_a(&self) -> &TextureCoordinate {
        &self.mesh.texcoords[self.a]
    }

    #[inline]
    pub fn texture_b(&self) -> &TextureCoordinate {
        &self.mesh.texcoords[self.b]
    }

    #[inline]
    pub fn texture_c(&self) -> &TextureCoordinate {
        &self.mesh.texcoords[self.c]
    }

    pub fn area(&self) -> f64 {
        let side1 = (self.c() - self.a()).length();
        let side2 = (self.c() - self.b()).length();
        let side3 = (self.b() - self.a()).length();

        let s = (side1 + side2 + side3) / 2.;

        (s * (s - side1) * (s - side2) * (s - side3)).sqrt()
    }
}
