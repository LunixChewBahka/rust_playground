// Control Flow, Conditionals and Pattern Matching
fn main() {
    // == =! < > <= >=

    let n = 7;
    let mut nn = 10;
    let mut cc = 0;
    let xx = 5;
    let xxx = 15;
    let pair = (0, -2); // a tuple
    let pair2 = (5, -5); // a tuple with guard example
    let p = 20;

    match xx {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    match xxx {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13...19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    match pair {
        (0, y) => println!("y: {}", y),
        (x, 0) => println!("x: {}", x),
        _      => println!("No match"),
    }

    match pair2 {
        (x, y) if x == y     => println!("Equal"),
        (x, y) if x + y == 0 => println!("Equal Zero"),
        (x, _) if x % 2 == 0 => println!("X is even"),
        _                    => println!("No match!"),

    }

    let np = match p {
        np @ 1 ... 12 => np,
        np @ 13 ... 19 => np,
        _ => 0,
    };

    println!("np: {}", np);

    for i in 0..101 { // #101 is not included
        println!("i: {}", i);
    }

    while nn != 0 {
        println!("{}!", nn);
        nn = nn - 1;
    }

    if n % 4 == 0 {
        println!("n is divisible by 4");
    } else if n % 3 == 0 {
        println!("n is divisible by 3");
    } else if n % 2 == 0 {
        println!("n is divisible by 2");
    } else {
        println!("n is not divisible by 4, 3 or 2");
    }

    let c = false;

    let n = if c {
        50 // true
    } else {
        76 // false
    };

    println!("n: {}", n);

    loop {
        println!("[{}] finite!", cc); // print finit
        cc += 1; // add 1 to counter
 
        if cc >= 10 { // if counter is more than or equal to 10
            break; // break out of the loop
        }
    }
    // end


}
