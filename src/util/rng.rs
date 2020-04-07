use rand::SeedableRng;
use rand_xoshiro::SplitMix64;
use std::cell::{RefCell, RefMut};

type RngType = SplitMix64;

thread_local! {
    static RNG: RefCell<RngType> = RefCell::new(RngType::from_entropy())
}

pub fn get_rng<T>(mut func: impl FnMut(RefMut<RngType>) -> T) -> T {
    RNG.with(|rng| {
        let r = rng.borrow_mut();
        func(r)
    })
}
