fn main() {
    let my_string = "Wassup Epic Toilet Paper Mic.".to_string();
    // pass my_string as reference to string
    takes_slice(&my_string);
}

fn takes_slice(slice: &str) {
    println!("Our Converted Slice: {}", slice);
}
