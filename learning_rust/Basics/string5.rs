// string5.rs
fn main() {
    let mut s = String::new();
    // s is initially empty but is already allocated on the heap
    s.push('H'); // push a single char 'H' to vec
    s.push_str("ello"); // push a string vec to the existing container `s`
    s.push(' '); // push a single char 'whitespace'
    s += "World!"; // append a String vec to s; short for `push_str`
    // by now it the output should be Hello World!
    // removes the '!' / exclamation point / char;
    s.pop();
    // push '?'
    s.push('?');
    assert_eq!(s, "Hello World"); // panic Hello World != Hello World
    println!("{}", s);
}
