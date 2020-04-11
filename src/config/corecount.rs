use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
// allow because names here are converted to yml keys which I want lowercase
#[allow(non_camel_case_types)]
pub enum ThreadCount {
    /// use all cores
    all,

    /// Number of threads to be used
    count(usize),

    /// Number of threads left over when running
    left(usize),
}

impl ThreadCount {
    pub fn get_cores(&self) -> usize {
        let num_cpus = num_cpus::get();

        match self {
            ThreadCount::left(threads) => num_cpus - *threads,
            ThreadCount::count(threads) => *threads,
            ThreadCount::all => num_cpus,
        }
    }
}

impl Default for ThreadCount {
    fn default() -> Self {
        ThreadCount::all
    }
}
