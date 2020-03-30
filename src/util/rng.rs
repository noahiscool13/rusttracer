use xorshift::{Rand, SeedableRng, Xorshift1024, Rng};
use std::time::{SystemTime, UNIX_EPOCH};
use std::cell::RefCell;

thread_local! {
    static RNG: RefCell<Xorshift1024> = {
        let now = SystemTime::now();
        let millis = now.duration_since(UNIX_EPOCH)
            .expect("Time went backwards (rerun)").as_nanos() as u64;

        let states = [millis, millis, millis, millis, millis, millis, millis, millis, millis, millis, millis, millis, millis, millis, millis, millis];
        RefCell::new(SeedableRng::from_seed(&states[..]))
    }
}


pub fn random_f64() -> f64 {

    RNG.with(|f| {
        f.borrow_mut().next_f64()
    })
}