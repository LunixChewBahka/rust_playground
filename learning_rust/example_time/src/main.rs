use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now();
    let seconds = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("{}", seconds);
}
