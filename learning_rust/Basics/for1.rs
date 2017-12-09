// for1.rs
fn main() {
    // range is not inclusive, it's like actualy range - 1 (end)
    // simple terms goes from 0-4 instead of 5
    for i in 0..5 {
        println!("Hello {}",  i);
    }
}
