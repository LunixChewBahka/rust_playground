//use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("lasat element of the slice: {:?}", slice.last());
    println!("the slice has {} elements", slice.len());

}
fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];
    println!("{:?}", xs);
    
    println!("borrow a section of the array as a slice.");
    analyze_slice(&ys[1..100]);
}
