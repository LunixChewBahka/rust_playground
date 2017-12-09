// vec4.rs
fn main() {
    let mut v1 = vec![1, 10, 10, 5, 1, 2, 11, 2, 25, 11, 40];
    println!("Before {:?}", v1);
    // handy builtin function; sorts from least to greatest
    v1.sort();
    // need to sort first to eliminate duplicates
    v1.dedup();
    //assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
    // duplicates nuked
    println!("After {:?}", v1);
}
