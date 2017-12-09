// for3.rs
fn main() {
    for i in 0..5 {
        // same as ternary operation in C programming
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        println!("{} {}", even_odd, i);
    }
}
