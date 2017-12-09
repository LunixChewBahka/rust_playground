// vec3.rs
fn main() {
    // shorthand
    let mut v1 = vec![10, 20, 30, 40];
    // remove 40 from the Vector
    v1.pop();

    // less abstraction
    let mut v2 = Vec::new();
    // push values to Vector
    v2.push(10);
    v2.push(20);
    v2.push(30);

    // both have the same ouput but differenct declaration
    assert_eq!(v1, v2);
    println!("{:?} {:?}", v1, v2);

    // using the extend() vec function we are able to add a range of numbers
    // from 0..2 (2 excluded)
    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);
    println!("{:?}", v2);
}
