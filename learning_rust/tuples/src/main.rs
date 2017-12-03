/*
   Structs
        A struct in another form of a record type, just like a tuple. There's
    a difference: structs give each element that they contain a name, called a
    field.
*/
struct Point {
    x: i32,
    y: i32,
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
