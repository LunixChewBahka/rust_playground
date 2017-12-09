fn main() {
    let s = String::from("hello"); // 's' comes into scope.
    // 's' value moves into the function ....
    takes_ownership(s);
    // .. and so is no longer valid here.

    let x = 5;      // x comes into scope.

    makes_copy(x);  // x would move into the function, but i32 is Copy
                    // so its okay to still use x afterward.
    println!("{}", s); // 's' is non-existent
    println!("{}", x); // x is still alive! 
} // Here, x goes out of scope, then s. But since s's value was moved, nothing special happens

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed. in short 's' dies here lol

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes our of scope. Nothing special happens.
