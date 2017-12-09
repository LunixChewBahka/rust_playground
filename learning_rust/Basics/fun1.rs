use std::f64::consts;
// fun1.rs

// fn sqr expects a floating point value / arg 
fn sqr(x: f64) -> f64 {
    // does the calculation and returns the result as f64
    // no need for 'return' or ';'
    // implicitly typed
    x * x
    // one can type the return statement explicitly like
    // return x * x;
}

// a few more example of this no-return expression style:
// absolute value of a flocating-point number
fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

//ensure the number always falls in the given range
fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

// factorial
fn factorial(n: u64) -> u64 {
    // base condition
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

// value can also be passed by reference. A reference is created by `&` and dereference by `*`.
fn by_ref(x: &i32) -> i32 {
    *x + 1
}

// fun4
fn modifies(x: &mut f64) {
    *x = 20.99;
}

fn main() {
    // pass a floating-point number to sqr
    let res = sqr(9475.42);
    let res_abs = abs(-33.3);
    let res_clamp = clamp(2.0, 3.0, 5.0);
    println!("square is {}", res);
    println!("abs value is {}", res_abs);
    println!("clamp result {}", res_clamp);

    let res_fact = factorial(3);
    println!("Factorial result {}", res_fact);

    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1, res2);

    let mut res_mod = 0.0;
    modifies(&mut res_mod);
    println!("res is {}", res_mod);

    let pi = consts::PI;
    let x = pi/2.0;
    let cosine = x.cos();
    println!("{} {} {}", pi, x, cosine);
}
