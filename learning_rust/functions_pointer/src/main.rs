// function pointers don't have parameters

fn main() {
    let x = hello_world;
    hello_world();
    x(); // amazing!
}

fn hello_world() {
    println!("Hello World");
}
