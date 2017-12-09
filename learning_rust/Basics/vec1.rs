// vec1.rs
fn main() {

    // initialize a new Vec called v which is mutable
    // we need to initialize v as a mutable vector because
    // we are going to push elements later
    let mut v = Vec::new();
    // push every element to the new Vec
    v.push(10);
    v.push(20);
    v.push(30);

    // get the value of the first element of the vector
    let first = v[0];       // will panic if out-of-range
    let maybe_first = v.get(0); // will return Some(actual value)

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
}
