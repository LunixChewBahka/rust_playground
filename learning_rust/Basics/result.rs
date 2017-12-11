// Result<>
fn main() {
    let x: Result<i32, &str> = Ok(-3);
    println!("{:?}", x);
    
    let x: Result<i32, &str> = Err("Some error message");
    println!("{:?}", x);
}
