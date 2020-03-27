use crate::scene::scene::{Mesh};
use crate::util::vector::Vector;
use crate::scene::material::Material;
use crate::scene::texturecoordinate::TextureCoordinate;

#[derive(Debug, Clone)]
pub struct Triangle<'t> {
    pub(super) a: usize,
    pub(super) b: usize,
    pub(super) c: usize,

    pub mesh: &'t Mesh<'t>
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


    pub fn index_a(&self) -> usize { self.a }
    pub fn index_b(&self) -> usize { self.b }
    pub fn index_c(&self) -> usize { self.c }
}