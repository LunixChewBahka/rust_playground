fn main() {
    let mut x = 17; // what is the type of this variable by default?
    println!("x = {}", x);
    x = my_func();
    println!("x = {}", x);
    x = calc(100, 100);
    println!("100 times 100 = {}", x);
}

fn my_func() -> i32 {
    let x = 9;
    x // in Rust, no semi-colon for returns; returns type of i32
}

fn calc(x:i32, y:i32) -> i32 {
    let result = x * y;
    result // no semi-colon again
}
