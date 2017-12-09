// sum1.rs
fn main() {
    // the trick is to always be explicit when declaraing variable types
    // and you don't want to screw yourself up later on the project
    let sum: i32 = (0..5).sum();
    // sum was 10
    println!("sum was {}",  sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    // sum was 60
    println!("sum was {}", sum);
}
