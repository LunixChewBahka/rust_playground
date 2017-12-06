#[macro_use]
extern crate text_io;

fn main() {
    let i: i32 = read!();
    println!("{}", i);

    let x: i32 = read!("You should {}!");
    println!("{:?}", x);
}
