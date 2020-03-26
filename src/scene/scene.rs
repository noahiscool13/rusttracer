use crate::util::vector::Vector;
use crate::scene::texture::{TextureAtlas, TextureAtlasBuilder};
use crate::scene::error::SceneError;
use std::path::Path;
use crate::scene::triangle::Triangle;
use crate::scene::material::Material;
use crate::scene::material::DEFAULT_MATERIAL;
use std::pin::Pin;
use std::mem;
use crate::scene::texturecoordinate::TextureCoordinate;


#[derive(Debug)]
pub struct Mesh<'m> {
    pub vertices: Box<[Vector]>,
    pub normals: Box<[Vector]>,
    pub triangles: Box<[Triangle<'m>]>,
    pub texcoords: Box<[TextureCoordinate]>,

    pub material: &'m Material<'m>,
}

impl<'m> Default for Mesh<'m> {
    fn default() -> Self {
        Self {
            vertices: vec![].into_boxed_slice(),
            normals: vec![].into_boxed_slice(),
            triangles: vec![].into_boxed_slice(),
            texcoords: vec![].into_boxed_slice(),
            material: &DEFAULT_MATERIAL,
        }
    }
}

/// Scene holds all the data about the scene (duh). It's stored in meshes.
/// Each mesh has the same material. meshes and materials are pinned because they are both
/// ridiculously self-referential. Each triangle holds a pointer to the mesh it's associated to,
/// and each mesh holds a pointer to the material. That's why some unsafe code is used.
pub struct Scene<'s> {
    textureatlas: TextureAtlas<'s>,

    meshes: Pin<Box<[Mesh<'s>]>>,
    materials: Pin<Box<[Material<'s>]>>,
}


impl<'s> Scene<'s> {
    pub fn triangles(&self) -> impl Iterator<Item = &Triangle>{
        self.meshes.iter()
            .flat_map(move |i| {
                i.triangles.iter()
            })
    }
}

/// Builds a scene from an object representation. For example [tobj](docs.rs/tobj)
pub struct SceneBuilder<'s> {
    /// This path is used to search for texture files.
    texturepath: &'s Path
}

impl<'s> SceneBuilder<'s> {
    pub fn new() -> Self{
        Self {
            texturepath: Path::new("")
        }
    }

    pub fn texturepath(mut self, path: &'s Path) -> Self {
        self.texturepath = path;
        self
    }

    pub fn build_from_tobj<'a>(&self, (models, tobjmaterials): (Vec<tobj::Model>, Vec<tobj::Material>)) -> Result<Scene<'a>, SceneError> {

        let mut meshes =  {
            let mut v = Vec::new();
            v.resize_with(models.len(), Default::default);

            Pin::new(v.into_boxed_slice())
        };
        let mut textureatlasbuilder = TextureAtlasBuilder::new();

        for i in &tobjmaterials {
            if !i.diffuse_texture.is_empty() {
                textureatlasbuilder.add_texture_file(&i.diffuse_texture, &self.texturepath)?
            }
            if !i.ambient_texture.is_empty() {
                textureatlasbuilder.add_texture_file(&i.ambient_texture, &self.texturepath)?
            }
            if !i.dissolve_texture.is_empty() {
                textureatlasbuilder.add_texture_file(&i.dissolve_texture, &self.texturepath)?
            }
            if !i.specular_texture.is_empty() {
                textureatlasbuilder.add_texture_file(&i.specular_texture, &self.texturepath)?
            }
        }

        let textureatlas = textureatlasbuilder.build();



        let materials = {
            let mut materials = Vec::new();

            for i in tobjmaterials {
                materials.push(unsafe {Material::from_tobj_material(i, &textureatlas)})
            }

            Pin::new(materials.into_boxed_slice())
        };

        for (index, model) in models.iter().enumerate() {
            let vertices = model.mesh.positions.chunks_exact(3)
                .map(|i| Vector::new(i[0] as f64, i[1] as f64, i[2] as f64));
            let triangles = model.mesh.indices.chunks_exact(3)
                .map(|i| {
                    let ptr: &'a Mesh = unsafe { mem::transmute(&meshes[index]) };
                    Triangle {
                        a: i[0] as usize,
                        b: i[1] as usize,
                        c: i[2] as usize,
                        mesh: ptr,
                    }
                });
            let normals = model.mesh.normals.chunks_exact(3)
                .map(|i| Vector::new(i[0] as f64, i[1] as f64, i[2] as f64));
            let texcoords = model.mesh.texcoords.chunks_exact(2)
                .map(|i| TextureCoordinate::new(i[0] as f64, i[1] as f64));

            let material = match model.mesh.material_id {
                Some(id) => {
                    let ptr: &'a Material = unsafe { mem::transmute(&materials[id]) };
                    ptr
                },
                None => &DEFAULT_MATERIAL
            };

            meshes[index] = Mesh {
                vertices: vertices.collect::<Vec<_>>().into_boxed_slice(),
                triangles: triangles.collect::<Vec<_>>().into_boxed_slice(),
                normals: normals.collect::<Vec<_>>().into_boxed_slice(),
                texcoords: texcoords.collect::<Vec<_>>().into_boxed_slice(),
                material,
            }
        }

        Ok(Scene {
            textureatlas,
            meshes,
            materials,
        })
    }
}



