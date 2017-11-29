// boolean = it's either 1 or 0; true or false; status flagging

fn main() {
    let result = is_number_greater_than(100, 55);
    println!("{}", result);
}

fn is_number_greater_than(x:i32, y:i32) -> bool { // returns a boolean value
    if x > y {
        true
    } else {
        false
    } // no semi-colon because we are returning a value in rust
}
