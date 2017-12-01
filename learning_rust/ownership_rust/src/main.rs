fn main() {
    let v = vec![3, 4, 5];
    take(v); // the function took ownership of the data
    //println!(":?", v[1]);
}


fn take(v: Vec<i32>) -> () {
    // v is here
    println!("{:?}", v[0]);
}
