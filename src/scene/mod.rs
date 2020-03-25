use crate::util::triangle::Triangle;
use crate::util::vector::Vector;
use tobj::Material;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref DEFAULT_MATERIAL: Material = Material {
        name: Default::default(),
        ambient: [0f32, 0f32, 0f32],
        diffuse: [0f32, 0f32, 0f32],
        specular: [0f32, 0f32, 0f32],
        shininess: 0.0,
        dissolve: 0.0,
        optical_density: 0.0,
        ambient_texture: Default::default(),
        diffuse_texture: Default::default(),
        specular_texture: Default::default(),
        normal_texture: Default::default(),
        dissolve_texture: Default::default(),
        illumination_model: None,
        unknown_param: Default::default()
    };
}

#[derive(Copy, Clone, Debug)]
pub enum Face {
    TOBJ {
        modelindex: usize,
        faceindex: usize,
    }
}

impl Face {
    pub fn material<'a>(&self, scene: &'a Scene) -> &'a Material {
        match self {
            Face::TOBJ {modelindex, faceindex} => {
                if let Scene::TOBJ((models, materials)) = scene {
                    let mesh = &models[*modelindex].mesh;
                    if let Some(id) = mesh.material_id {
                        &materials[id]
                    } else {
                        &DEFAULT_MATERIAL
                    }
                } else {
                    &DEFAULT_MATERIAL
                }
            }
        }
    }

    pub fn positions(&self, scene: &Scene) -> (Vector, Vector, Vector){
        match self {
            Face::TOBJ { modelindex, faceindex } => {
                if let Scene::TOBJ((models, materials)) = scene {
                    let mesh = &models[*modelindex].mesh;

                    let a = mesh.indices[faceindex * 3 + 0];
                    let b = mesh.indices[faceindex * 3 + 1];
                    let c = mesh.indices[faceindex * 3 + 2];

                    let ax = mesh.positions[a as usize * 3 + 0] as f64;
                    let ay = mesh.positions[a as usize * 3 + 1] as f64;
                    let az = mesh.positions[a as usize * 3 + 2] as f64;

                    let bx = mesh.positions[b as usize * 3 + 0] as f64;
                    let by = mesh.positions[b as usize * 3 + 1] as f64;
                    let bz = mesh.positions[b as usize * 3 + 2] as f64;

                    let cx = mesh.positions[c as usize * 3 + 0] as f64;
                    let cy = mesh.positions[c as usize * 3 + 1] as f64;
                    let cz = mesh.positions[c as usize * 3 + 2] as f64;


                    (
                        Vector::new(ax, ay, az),
                        Vector::new(bx, by, bz),
                        Vector::new(cx, cy, cz),
                    )
                } else {
                    (
                        Vector::new(0f64, 0f64, 0f64),
                        Vector::new(0f64, 0f64, 0f64),
                        Vector::new(0f64, 0f64, 0f64),
                    )
                }
            }
        }
    }

    pub fn normal(&self, scene: &Scene) -> Vector {
        // TODO: depends on illum model
        let (a, b, c) = self.positions(scene);

        (c-a).cross(c-b)
    }
}

pub enum Scene {
    TOBJ((Vec<tobj::Model>, Vec<tobj::Material>)),
}

impl Scene {
    pub fn triangles<'a>(&'a self) -> impl Iterator<Item=Triangle> + 'a {
        match self {
            Scene::TOBJ((models, materials)) => {
                models.iter()
                    .enumerate()
                    .flat_map(|(modelindex, model)| {
                        let mesh = &model.mesh;
                        mesh.indices.chunks_exact(3)
                            .enumerate()
                            .map(move |(faceindex, chunk)| {
                                match *chunk {
                                    [a, b, c] => {
                                        let ax = mesh.positions[a as usize * 3 + 0] as f64;
                                        let ay = mesh.positions[a as usize * 3 + 1] as f64;
                                        let az = mesh.positions[a as usize * 3 + 2] as f64;

                                        let bx = mesh.positions[b as usize * 3 + 0] as f64;
                                        let by = mesh.positions[b as usize * 3 + 1] as f64;
                                        let bz = mesh.positions[b as usize * 3 + 2] as f64;

                                        let cx = mesh.positions[c as usize * 3 + 0] as f64;
                                        let cy = mesh.positions[c as usize * 3 + 1] as f64;
                                        let cz = mesh.positions[c as usize * 3 + 2] as f64;

                                        Triangle::new(
                                            Vector::new(ax, ay, az),
                                            Vector::new(bx, by, bz),
                                            Vector::new(cx, cy, cz),
                                            Face::TOBJ {
                                                modelindex,
                                                faceindex
                                            }
                                        )
                                    }
                                    _ => unreachable!()
                                }
                            })
                    })
            }
        }
    }
}
