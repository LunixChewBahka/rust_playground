extern crate rand;

use rand::{Rng, SeedableRng, StdRng};

fn main() {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    println!("{}", rng.gen::<f64>());
    rng.reseed(&[5, 6, 7, 8]);
    println!("{}", rng.gen::<f64>());
    rng.reseed(&[10, 11, 12, 13]);
    println!("{}", rng.gen::<f64>());
}
