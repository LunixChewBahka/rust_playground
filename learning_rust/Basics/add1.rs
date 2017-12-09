// add1.rs
fn main() {
    // let declares variable as immutable by default,
    // so we have to make it mutable if we want it to change at runtime
    let mut sum = 0.0; // try changing to 0 to 0.0
    for i in 0..5 {
        // re-assignment of immutable variable
        // cast the value to floating point to be able to compile the program
        sum += i as f64;
    }
    println!("Sum is {}", sum);
}
