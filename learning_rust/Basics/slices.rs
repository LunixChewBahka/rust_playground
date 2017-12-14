// slices does not have ownership
// slices let you reference a contingous sequence of elements in a collection rather than the whole collection

fn first_word(s: &str) -> &str {
    // convert string to an array of bytes using as_bytes method:
    let bytes = s.as_bytes();

    // next, we create an iterator over the array of bytes using the iter method:
    for (i, &item) in bytes.iter().enumerate() {
        // we search for the byte that represents the space by using byte literal syttax. if we find a space, we return the position. otherwise, we return the length of the string by using s.len();
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice_a = &a[0..2];
    println!("a: {:?}", a);
    println!("slice_a: {:?}", slice_a);
    // String Literals Are Slices
    //let s = "Hello, World!";
    //let word = first_word(s); // word will get the value 5.

    let my_string = String::from("hello world");
    // first_word works on slices of `Strings`s
    let word2 = first_word(&my_string[..]);
    println!("my_string[..] word2 {}", word2);

    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word3 = first_word(&my_string_literal[..]);
    println!("my_string_literal[..] word3 {}", word3);

    // since string literals *are* string slices already,
    // this words too, without the slice syntax!
    let word4 = first_word(my_string_literal);
    println!("my_string_literal[..] word4 {}", word4);

    //s.clear(); // this empties the String, making it equal to "".

    // word still has the value 5 here, but there's no more string that
    // we coud meaningfully use the valu 5 with. word in now totally invalid
    //println!("Size of the first word: {}", word);

    let ss = String::from("Hello World");
    let hello = &ss[0..5];
    let world = &ss[6..11];
    println!("{} {}", hello, world);

    let sli = String::from("hello");
    let sli1 = &sli[0..sli.len()];
    let sli2 = &sli[..];
    println!("{} {}", sli1, sli2);
}
