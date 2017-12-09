// string3.rs
fn main() {
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}'", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }

    let s = "i";
    println!("{}", &s[0..1]);

    let text = "The red fox and the Lazy Dog";
    // 1st way of using collect()
    let words_1: Vec<&str> = text.split_whitespace().collect();
    println!("{:?}", words_1);

    // 2nd way of using collect()
    let mut words_2 = Vec::new();
    words_2.extend(text.split_whitespace());
    println!("{:?}", words_2); // same output as words_1

    /*
       In most languages, we would have to make these separately allocated strings,
       whereas here each slice inthe vector is borrwing from the original string. All we allocate is the space to keep the slices.
    */

    let stripped_text: String = text.chars()
        .filter(|ch| ! ch.is_lowercase()).collect();
    println!("{:?}", stripped_text);
    // you can also do this as an explicit loop over every char,
    // pushing returned slices into a mutable vector,
    // but the provided example above is more elegent and short
    // using filter method which takes a closure
}
