use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum CoreCount {
    /// dummy value to make it serializable
    all,

    /// Number of cores to be used
    count(usize),

    /// Number of cores left over when running
    left(usize),
}

impl Default for CoreCount {
    fn default() -> Self {
        // CoreCount::all

        CoreCount::left(5)
    }
}