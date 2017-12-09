// string1.rs
fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    let text = "hello dolly"; // the string slice; Type: &str
    let s = text.to_string(); // it's now an allocated String

    dump(text);
    dump(&s);

    // The borrow operator can coerce String into &str
    // just as Vec into &[]; coerced somewhat force rather than voluntary
    /*
       under the hood, String is basically a Vec<u8> while &str is &[u8]
       both must represent valid UTF-8 text.
    */
}
