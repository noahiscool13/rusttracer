use crate::scene::scene::Scene;
use crate::scene::triangle::Triangle;
use rand::distributions::weighted::alias_method::WeightedIndex;
use rand::distributions::WeightedError;
use crate::util::rng::get_rng;
use rand::Rng;

#[derive(Debug)]
pub enum LightError {
    WeightedError(WeightedError),
}

pub struct LightSourceManager<'l> {
    lightsources: Vec<&'l Triangle<'l>>,
    weights: WeightedIndex<u64>,
}

impl<'l> LightSourceManager<'l> {
    pub fn new(scene: &'l Scene) -> Result<Self, LightError> {
        Self::from_triangle_iter(scene.triangles())
    }

    pub(super) fn from_triangle_iter(iter: impl Iterator<Item = &'l Triangle<'l>>) -> Result<Self, LightError> {
        let lightsources: Vec<&'l Triangle> = iter
            .filter(|i| !i.mesh.material.emittance.iszero())
            .collect();

        let weights = WeightedIndex::new(
            lightsources.iter()
                .map(|l| {
                    let area = l.area();
                    let emittance = l.mesh.material.emittance.length();

                    (area * emittance) as u64
                }).collect()
        ).map_err(LightError::WeightedError)?;

        Ok(Self {
            lightsources,
            weights,
        })
    }

    pub fn random_source(&self) -> &'l Triangle{
        let index = get_rng(|mut r| r.sample(&self.weights));
        self.lightsources[index]
    }
}


