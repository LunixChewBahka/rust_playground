fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // tuple declaration
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();       // len() returns the length of a string.
    (s, length)     // return the string that was passed and the length
}
