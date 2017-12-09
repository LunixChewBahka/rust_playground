use std::mem;
// string2.rs

fn main() {
    let text = "static";
    let string = "dynamic".to_string();

    println!("This slice occupies {} bytes of memory.", mem::size_of_val(&text));
    println!("This slice occupies {} bytes of memory.", mem::size_of_val(&string));

    let text_s = &text[1..];
    let string_s = &string[3..4];

    println!("{}", text.len());
    println!("{}", string.len());
    println!("This slice occupies {} byte(s) of memory.", mem::size_of_val(text_s));
    println!("This slice occupies {} byte(s) of memory.", mem::size_of_val(string_s));

    // tried so hard to index strings! but that's not possible :(
    // because they use what we call UTF-8 aka The One True Encoding
    // wherein a 'character' may be a number of bytes.

    println!("slices {:?} {:?}", text_s, string_s);
}
