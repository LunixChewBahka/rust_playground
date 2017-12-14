fn main() {
    let ab = 6;
    take(ab);
    println!("{}", ab);

    println!("Hello {}!", foo("bar"));
}

fn take(val: i32) {
    println!("{}", val);
}

// _x to avoid warning
fn foo(_x: &'static str) -> &'static str { // could also do () return type
    return "world";
    // return "world"
    // "world"
    // these are all the same
}
