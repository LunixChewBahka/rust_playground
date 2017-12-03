use std::cmp::Ordering;

enum OptionalInt {
    Value(i32),
    Missing,
}

fn cmp(a: i32, b:i32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b  { Ordering::Greater }
    else { Ordering::Equal }
}

fn main() {

    // somewhat like the 'switch' statement in C programming
    let z = 5;

    match z {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"), // more like the default condition
    }

    let a = 5;
    let b = 10;

    match cmp(a, b) {
        Ordering::Less => println!("Less"),
        Ordering::Greater => println!("Greater"),
        Ordering::Equal => println!("Equal"),
    }

    // match is also an expression, which means we can use it on the right-hand
    // side of a let binding or directly where an expression is used. We could
    // also implement the previous example like this:
    println!("{}", match cmp(a, b) {
        Ordering::Less => "Less",
        Ordering::Greater => "Greater",
        Ordering::Equal => "Equal",
    });
    // another version less println!'s which is nice

    let x = OptionalInt::Value(5);
    let y = OptionalInt::Missing;

    // very nice ^^,
    match x {
        OptionalInt::Value(n) => println!("x is {}", n),
        OptionalInt::Missing => println!("x is mising!"),
    }

    match y {
        OptionalInt::Value(n) => println!("y is {}", n),
        OptionalInt::Missing => println!("y is mising!"),
    }
}
