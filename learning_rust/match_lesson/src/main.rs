#![allow(unused_variables)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn value_in_cents(coin: Coin) -> (String, u32) {
    match coin {
        Coin::Penny => ("Lucky Penny".to_string(), 1),
        Coin::Nickel => ("Lucky Nickel".to_string(), 5),
        Coin::Dime => ("Lucky Dime".to_string(), 10),
        Coin::Quarter => ("Lucky Quarter".to_string(), 25),
    }
}


fn main() {
    println!("{:?}", value_in_cents(Coin::Penny));
    println!("{:?}", value_in_cents(Coin::Nickel));
    println!("{:?}", value_in_cents(Coin::Dime));
    println!("{:?}", value_in_cents(Coin::Quarter));
    
    let m = Message::Write(String::from("hello"));
    m.call();
}
