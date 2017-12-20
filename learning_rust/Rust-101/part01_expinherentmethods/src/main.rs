/**
  Rust-101, Part 01: Expression, Inherent methods
  For rust to compile this file, make sure to enable the corresponding line in main.rs before going on.

  Even though our code from the first aprt works, we can still learn a lot by making it prettier. That's because Rust is an "expression-based" language, which means that most of the terms you write down are not just statements(executing code), but expressions(returning a value). Thisa pplies even to the body of entire functions!

  Expression-based programming
  For example, consider sqr:

  Between the curly braces, we are giving the expression taht computes the return value. So we can just write i * i, the expression that returns the square of i! This is very close to how mathematicians write down functions (but with more types).

  Conditionals are also just expression. This is comparable to the ternary ? : operator from languages like C.

  And the same applies to case distinction with match: Every arm of the match gives the expression that is returned in the respective case.(We repeat the definition from the previous part here.)
  */

fn sqr(i: i32) -> i32 {
    i * i
}

fn abs(i: i32) -> i32 {
    if i >= 0 { 
        i
    } else {
        -i
    }
}

enum NumberOrNothing {
    Number(i32),
    Nothing
}

use self::NumberOrNothing::{Number,Nothing};

fn number_or_default(n: NumberOrNothing, default: i32) -> i32 {
    match n {
        Nothing => default,
        Number(n) => n,
    }
}

fn compute_stuff(x: i32) -> i32 {
    let y = { let z = x * x; z + 14 }; // a closure?
    y * y
}

fn vec_min(v: &Vec<i32>) -> NumberOrNothing {

    fn min_i32(a: i32, b: i32) -> i32 {
        if a < b { 
            a
        } else {
            b
        }
    }

    let mut min = Nothing;
    for e in v {
        min = Number(match min {
            nothing => *e,
            Number(n) => min_i32(n, *e)
        });
    }

    min
}

impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Number(n) => println!("The number is: {}", n),
        }; // since this is an expression should end with ;
    }
}

fn read_vec() -> Vec<i32> {
    vec![18,5,7,2,9,27]
}

fn vec_sum(n: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for e in n.iter() {
        sum += *e;
    }
    sum
}

fn vec_print(n: &Vec<i32>) -> () {
    for (elem, val) in n.iter().enumerate() {
        println!("Element[{:?}] is {:?}", elem, *val);
    }
}

fn main() {
    let vec = read_vec();
    let min = vec_min(&vec);
    // can borrow since vec in immutable (unlimited borrows)
    let sum = vec_sum(&vec);
    min.print();
    println!("Sum of all values of the Vec<i32>: {}", sum);
    vec_print(&vec);
}
