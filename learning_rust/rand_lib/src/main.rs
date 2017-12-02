extern crate rand;

use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

    let mut count = 0u32;

    loop {
        let n: u32 = rng.gen_range(0, 10);
        let m: f64 = rng.gen_range(-40.0f64, 1.3e5f64);
        println!("[{}] {} {}", count, n, m);

        count += 1;

        if count == 100 {
            println!("Time to stop. We have already reached {}", count);
            break;
        }

    }
}
