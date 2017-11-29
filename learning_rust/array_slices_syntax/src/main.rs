// slices have a type of generic<T>????
fn main() {
    let a = [52, 101, 99, 27, 36, 69]; // initialize array
    print!("{:?}", a); // print values
    let entire_array = &a[0..2]; // creates the view into the array
    println!("{:?}", entire_array);
}
