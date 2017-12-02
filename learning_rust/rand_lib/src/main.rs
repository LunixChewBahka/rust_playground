extern crate rand;

use rand::{thread_rng, Rng};

struct HasDrop;


impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    fn_fill_bytes();
    println!("-------------------- fill_bytes_example END -------------------- ");
    let mut rng = thread_rng();
    let mut count = 0u32;

    loop {
        let n: u32 = rng.gen_range(0, 10);
        let m: f64 = rng.gen_range(-40.0f64, 1.3e5f64);
        println!("[{}] {} {}", count, n, m);

        count += 1;

        if count == 5 {
            println!("New policy. Should stop at {}", count);
            break;
        }
    }

    let _x = HasDrop;
    drop(_x);

}

fn fn_fill_bytes() {
    let mut fb = [0u8; 100];
    thread_rng().fill_bytes(&mut fb);
    println!("{:?}", &fb[..]);
}
