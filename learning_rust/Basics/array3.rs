// array3.rs
fn main() {
    let ints = [1, 2, 3];
    let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    let var: () = [1, 1]; // check actual variable type declared by `()` as type
    println!("ints {:?}",  ints);
    println!("floats {:?}", floats);
    println!("string {:?}", strings);
    println!("ints_ints {:?}", ints_ints);
    println!("{:?}", var);
}
