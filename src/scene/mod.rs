pub mod error;
pub mod light;
pub mod material;
pub mod texture;
pub mod texturecoordinate;
pub mod triangle;

use crate::scene::error::SceneError;
use crate::scene::light::LightSourceManager;
use crate::scene::material::Material;
use crate::scene::material::DEFAULT_MATERIAL;
use crate::scene::texture::{TextureAtlas, TextureAtlasBuilder};
use crate::scene::texturecoordinate::TextureCoordinate;
use crate::scene::triangle::Triangle;
use crate::util::vector::Vector;
use serde::export::fmt::Debug;
use serde::export::Formatter;
use std::path::Path;
use std::pin::Pin;
use std::{fmt, mem};
use std::sync::Arc;

#[derive(Debug)]
pub struct Mesh<'m> {
    pub vertices: Box<[Vector]>,
    pub normals: Box<[Vector]>,
    pub triangles: Box<[Triangle<'m>]>,
    pub texcoords: Box<[TextureCoordinate]>,

    pub material: &'m Material<'m>,

    // Private by design. This option is actually always Some()
    lightsourcemanager: Option<Arc<LightSourceManager<'m>>>
}

impl<'m> Mesh<'m> {
    pub fn lightsourcemanager(&self) -> &Arc<LightSourceManager<'m>> {
        self.lightsourcemanager.as_ref().unwrap()
    }
}

impl<'m> Default for Mesh<'m> {
    fn default() -> Self {
        Self {
            vertices: vec![].into_boxed_slice(),
            normals: vec![].into_boxed_slice(),
            triangles: vec![].into_boxed_slice(),
            texcoords: vec![].into_boxed_slice(),
            material: &DEFAULT_MATERIAL,
            lightsourcemanager: None,
        }
    }
}

/// Scene holds all the data about the scene (duh). It's stored in meshes.
/// Each mesh has the same material. meshes and materials are pinned because they are both
/// ridiculously self-referential. Each triangle holds a pointer to the mesh it's associated to,
/// and each mesh holds a pointer to the material. That's why some unsafe code is used.
pub struct Scene<'s> {
    #[allow(unused)]
    textureatlas: TextureAtlas<'s>,

    #[allow(unused)]
    meshes: Pin<Box<[Mesh<'s>]>>,

    #[allow(unused)]
    materials: Pin<Box<[Material<'s>]>>,
}

impl<'s> Debug for Scene<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<Scene...>")
    }
}

impl<'s> Scene<'s> {
    pub fn triangles(&self) -> impl Iterator<Item = &Triangle> {
        self.meshes.iter().flat_map(move |i| i.triangles.iter())
    }

    #[allow(unused)]
    pub fn vertices(&self) -> impl Iterator<Item = &Vector> {
        self.meshes.iter().flat_map(move |i| i.vertices.iter())
    }

    #[allow(unused)]
    pub fn texture_coordinates(&self) -> impl Iterator<Item = &TextureCoordinate> {
        self.meshes.iter().flat_map(move |i| i.texcoords.iter())
    }

    #[allow(unused)]
    pub fn normals(&self) -> impl Iterator<Item = &Vector> {
        self.meshes.iter().flat_map(move |i| i.normals.iter())
    }
}

/// Builds a scene from an object representation. For example [tobj](docs.rs/tobj)
pub struct SceneBuilder<'s> {
    /// This path is used to search for texture files.
    texturepath: &'s Path,
}

impl<'s> SceneBuilder<'s> {
    pub fn new() -> Self {
        Self {
            texturepath: Path::new(""),
        }
    }

    pub fn texturepath(mut self, path: &'s Path) -> Self {
        self.texturepath = path;
        self
    }

    pub fn build_from_tobj<'a>(
        &self,
        (models, tobjmaterials): (Vec<tobj::Model>, Vec<tobj::Material>),
    ) -> Result<Scene<'a>, SceneError> {
        let mut meshes = {
            let mut v = Vec::new();
            v.resize_with(models.len(), Default::default);

            Pin::new(v.into_boxed_slice())
        };
        let mut textureatlasbuilder = TextureAtlasBuilder::new();

        for material in &tobjmaterials {
            if !material.diffuse_texture.is_empty() {
                textureatlasbuilder
                    .add_texture_file(&material.diffuse_texture, &self.texturepath)?
            }
            if !material.ambient_texture.is_empty() {
                textureatlasbuilder
                    .add_texture_file(&material.ambient_texture, &self.texturepath)?
            }
            if !material.dissolve_texture.is_empty() {
                textureatlasbuilder
                    .add_texture_file(&material.dissolve_texture, &self.texturepath)?
            }
            if !material.specular_texture.is_empty() {
                textureatlasbuilder
                    .add_texture_file(&material.specular_texture, &self.texturepath)?
            }

            let default_emittance_texture_name = "".into();
            let emittance_texture_name = material
                .unknown_param
                .get("map_Ke")
                .unwrap_or(&default_emittance_texture_name);

            if !emittance_texture_name.is_empty() {
                textureatlasbuilder.add_texture_file(emittance_texture_name, &self.texturepath)?
            }
        }

        let textureatlas = textureatlasbuilder.build();

        let materials = {
            let mut materials = Vec::new();

            for i in tobjmaterials {
                materials.push(unsafe { Material::from_tobj_material(i, &textureatlas) })
            }

            Pin::new(materials.into_boxed_slice())
        };

        for (index, model) in models.iter().enumerate() {
            let vertices = model
                .mesh
                .positions
                .chunks_exact(3)
                .map(|i| Vector::new(i[0] as f64, i[1] as f64, i[2] as f64));
            let triangles = model.mesh.indices.chunks_exact(3).map(|i| {
                let ptr: &'a Mesh = unsafe { mem::transmute(&meshes[index]) };
                Triangle {
                    a: i[0] as usize,
                    b: i[1] as usize,
                    c: i[2] as usize,
                    mesh: ptr,
                }
            });
            let normals = model
                .mesh
                .normals
                .chunks_exact(3)
                .map(|i| Vector::new(i[0] as f64, i[1] as f64, i[2] as f64));
            let texcoords = model
                .mesh
                .texcoords
                .chunks_exact(2)
                .map(|i| TextureCoordinate::new(i[0] as f64, i[1] as f64));

            let material = match model.mesh.material_id {
                Some(id) => {
                    let ptr: &'a Material = unsafe { mem::transmute(&materials[id]) };
                    ptr
                }
                None => &DEFAULT_MATERIAL,
            };

            meshes[index] = Mesh {
                vertices: vertices.collect::<Vec<_>>().into_boxed_slice(),
                triangles: triangles.collect::<Vec<_>>().into_boxed_slice(),
                normals: normals.collect::<Vec<_>>().into_boxed_slice(),
                texcoords: texcoords.collect::<Vec<_>>().into_boxed_slice(),
                material,
                lightsourcemanager: None,
            }
        }

        let lightsourcemanager = Arc::new(LightSourceManager::from_triangle_iter(
            meshes
                .iter()
                .flat_map(move |i| i.triangles.iter())
                .map(|i| {
                    let ptr: &'a Triangle = unsafe { mem::transmute(i) };
                    ptr
                }),
        )?);

        for i in meshes.iter_mut() {
            i.lightsourcemanager = Some(lightsourcemanager.clone())
        }


        Ok(Scene {
            textureatlas,
            meshes,
            materials,
        })
    }
}
