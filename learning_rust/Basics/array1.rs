// array1.rs
fn main() {
    let arr = [10, 20, 30, 40];
    // the type of this array is [i32; 4]
    // meaning the array has 4 elements which is of type i32
    // given an array [10, 20]; the type is [i32; 2]
    let first = arr[0];
    let last = arr[3];
    println!("first {}", first);
    println!("last {}", last);

    for i in 0..4 {
        println!("[{}] = {}", i, arr[i]);
    }
    println!("length of array is: {}", arr.len());
}
