// Strings, Tuples, Arrays, Slices and Pretty Printing

fn main() {
    let xs: [i32; 5] = [4, 5, 6, 7, 8];
    let ys = &xs[2..4];
    println!("{:?} {:?}", ys, xs);

    let s = "String"; // Type of &str aka a string slice
    let ss = String::from("String!"); // Type of String... compound types of of multiple diff chars
    let conv_s = s.to_string(); // converts &str to a String output: Stri

    let slice = &ss[0..4];
    println!("{}", slice);

    let h = String::from("Hello, ");
    let w = String::from("World!");
    let s = h + &w;
    println!("{}", s);

    let t1 = (); // by default a fn in rust return a tuple if no return type is specified
}
