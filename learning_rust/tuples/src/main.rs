/*
   Structs
        A struct in another form of a record type, just like a tuple. There's
    a difference: structs give each element that they contain a name, called a
    field.
*/

use std::cmp::Ordering::{self, Equal, Less, Greater};

fn cmp(a: i32, b: i32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

// basic struct implementation
struct Point {
    x: i32,
    y: i32,
}

// tuple struct implementation; no field names
struct Colorv2(i32, i32, i32);
struct Pointv2(i32, i32, i32);

// tuple struct with 1 element
struct Inches(i32);

// struct implementation rather than tuple struct; with field names
struct Colorv3 {
    red: i32,
    blue: i32,
    green: i32,
}

struct Pointv3 {
    x: i32,
    y: i32,
    z: i32,
}

// enums aka "sum type"
enum Character {
    Digit(i32),
    Other,
}

fn main() {
    let my_tuple = ('k', 45);

    println!("Our tuple contains the letter: {:?}", my_tuple.0);
    println!("Our tuple contains the number: {:?}", my_tuple.1);

    let (a,b) = my_tuple; // declared two separate variables for destructuring
    println!("Fancy: {:?}", a);
    println!("Pancy: {:?}", b);

    let xna = (1, "hello I am not type annotated");
    let xta: (i32, &str) = (1, "hello I am type annotated"); //type annotated

    println!("{:?}", xna);
    println!("{:?}", xta);
    // tuple destructuring in rust e.g. : (435, 'f', another_object)
    // what if we want to store each value in a variable? ans.tuple destructuring sytax

    // another version of tuple destructuring
    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);

    // we can assign a tuple into another tuple
    let mut tx = (1, 5);    // x: (i32, i32)
    let ty = (2, 10);        // y: (i32, i32)

    println!("Before {:?}", tx);
    tx = ty;
    println!("After {:?}", tx);

    let t1 = (2, 2, 4);
    let t2 = (2, 2, 3);

    if t1 == t2 {
        println!("{:?}", true);        
    } else {
        println!("{:?}", false);        
    }
    println!("{:?}", pass_and_compare_tuples(t1, t2));

    let (g, h) = next_two(5);
    println!("g, h = {:?}, {:?}", g, h);

    sum(prepare_args());

    let origin = Point { x: 22, y: 100 };      // origin: Point
    println!("The origin is at ({}, {})", origin.x, origin.y);

    let mut mut_origin = Point { x: 22, y: 100 };      // origin: Point
    mut_origin.x = 99;
    mut_origin.y = 99;
    println!("The mutated origin is at ({}, {})", mut_origin.x, mut_origin.y);

    let blackv2 = Colorv2(0, 0, 0);
    let originv2 = Pointv2(0, 0, 0);

    let mut greenv3 = Colorv3 { red: 0, blue: 0, green: 0};
    println!("Original greenv3 ({} {} {})", greenv3.red, greenv3.blue, greenv3.green);
    greenv3.red = 0;
    greenv3.blue = 255;
    greenv3.green = 255;
    println!("Original greenv3 ({} {} {})", greenv3.red, greenv3.blue, greenv3.green);


    let length = Inches(10);

    let Inches(integer_length) = length;
    println!("length is {:?} inches", integer_length);

    // These assignment both succeed
    let ten = Character::Digit(10);
    let four = Character::Digit(4);

    // Error: `*` is not implement for type `Character`
    //let forty = ten * four;

    // Error: `<=` is not implemented for type `Character`
    //let four_is_smaller = four <= ten;

    // Error: `==` is not implemented for type `Character`
    //let four_equals_ten = four == ten;
    let xo = 100;
    let yo = 10;

    let ordering = cmp(xo, yo);     // ordering: ordering

    if ordering == Ordering::Less { println!("less"); }
    else if ordering == Ordering::Greater { println!("Greater"); }
    else if ordering == Ordering::Equal { println!("Equal"); }
}

fn pass_and_compare_tuples(t1: (i32, i32, i32), t2: (i32, i32, i32)) -> bool {
    if t1 == t2 {
        true
    } else {
        false
    }
}

fn next_two(x: i32) -> (i32, i32) { // return 2 results
    (x + 1, x + 2) 
}

fn sum((x, y): (i32, i32)) -> i32 {
    x + y
}

fn prepare_args() -> (i32, i32) {
    (1, 2)
}
