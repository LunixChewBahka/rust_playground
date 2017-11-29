fn main() {
    // Add two numbers
    println!("{:?}", add_numbers(7, 7));
}

// fn = function - is a reserved keyword
fn add_numbers(x: i32, y: i32) -> i32 { // takes 2 i32 values as parameters and returns an i32
    //implicit return
    // x + y   // return the result of x and y
    // explicit
    return x + y; // with 'return keyword'
}
