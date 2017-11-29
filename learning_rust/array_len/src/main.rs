fn main() {
    let z = [1, 2, 3];
    println!("{:?}", z.len());
    greet_user("Jar");
}

fn greet_user(user: &str) {
    println!("Hello {}!", user);
}
