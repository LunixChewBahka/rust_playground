// slice1.rs
fn main() {
    let ints = [1, 2, 3, 4, 5];
    // kinda of like the same with python but the difference is that in rust a copy of the data is never made!!!
    // the power of borrow (exclusive access concept; hence no need for copying)
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];        // wtf and open range!; up until the last element; which is kinda neat 

    println!("ints {:?}",  ints);
    println!("slice1 {:?}",  slice1);
    println!("slice2 {:?}",  slice2);
}
