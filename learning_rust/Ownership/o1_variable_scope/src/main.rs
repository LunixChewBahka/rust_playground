/*
   Ownership Rules:
       1. Each value if Rust has a variable that's called its owner.
       2. There can only be one owner at a time.
       3. When the owner goes out of scope, the valu will be dropped.
*/
fn main() { // 's' is not valid here, it's not yet declared

    // Scope of some variables. A scope is the range within a program which an
    // item is valid.
    let mut s = String::from("Hello"); // 's' is valid from this point forward

    s.push_str(", World!");     // push_str() appends a  literal to a String

    println!("{}", s);          // This will print `hello, world!`
    // do something with 's'

    // no need to free s1 since the value was moved to s2
    // so only s2 is needed to be free'd
    let s1 = String::from("String 1");
    let s2 = s1.clone();
    
    println!("s1 = {}, s2 = {}", s1, s2);
}   // this scope is now over, and s is no longer valid.

/*
   There are two important points in time here:
        1. When s comes into scope, it is valid.
        2. It remains so until it goes out of scope.

    With the String tupe, in order to support a mutable, growable piece of text,
    we need to allcoate an amount of memory on the heap, unkown at compile time,
    to hold the contents. This means:
        1. The memory must be requested from the operating system at runtime.
        2. We need of returning this memory to the operating system when we're
        done with out String.

    &str; string literals are faster compared to String types
*/
