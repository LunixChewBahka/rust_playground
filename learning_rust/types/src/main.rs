/*
   bool, char, Numeric types, Arrays, Slices, Tuples, str, &str

            Signed  Unsigned    Floating
   8        i8      u8          -
   16       i16     u16         -
   32       i32     u32         f32
   64       i64     u64         f64
   Machine  isize   usize       -

   Traits

   A trait is a language feature that tells the Rust compiler about functionality
   a type must provide    -- The Rust Book

   Useful types of the stdlib

   Option, Result, Vec, HashMap

   Vec - a global array, vec's are dynamic; a fat pointer?

   HashMap - 

   Useful traits from the stdlib
        From, Into - FromStr - Debug, Display
*/

use std::f32::consts::PI;
use std::collections::HashMap;
use std::io;

// Simplefied definition of Option
enum Option<T> {
    None,
    Some(value: T),
}

enum Result<T, E> {
    Ok(value: T),
    Err(error: E),
}

// Custom types, structs can be used inside impl's
struct Complex {
    real: i32,
    imag: i32,
}

struct Circle { 
    radius: f32,
}

struct Square {
    side_length: f32,
}

// One can use traits instead of Generics, they seem to be easier in the eyes
pub trait Add {
    fn add(self, rhs: Self) -> Self;
}

trait Shape {
    fn area(&self) -> f32;
}

// impl is easier to work with
impl Complex {
    fn new(real: i32, imag: i32) -> Complex {
        Complex { real: real, imag: imag }
    }

    fn zero() -> Complex {
        Complex { real: 0, imag: 0 }
    }
}

impl Add for Complex {
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real,
                     self.imag + other.imag)
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        PI * self.radius* self.radius
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.side_length.powi(2)
    }
}

impl From<(i32, i32)> for Complex {
    fn from(pair: (i32, i32)) -> Complex {
        Complex::new(pair.0, pair.1);
    }
}

fn main() {
    let greeting: &str = "Hello, World!"; // explicit
    let count: i32 = 3; //  explicit
    for i in 0..count {
        println!("{}: {}", i, greeting);
    }

    println!("{}", toy(3, 4)); // 10

    let c = Complex { real: 1, imag: 2 };
    let c1 = Complex::new(1, 2);
    let c2 = Complex::new(3, 4);

    toy(c1, c2); // 5+8i

    let sc = Circle { radius: 10 };
    let ss = Square { side_length :5 };

    big_enough(&c); // should be true
    big_enough(&s); // should be false

    let squares: Vec<i32> = Vec::new();
    for n in 0..10 {
        squares.push(n * 2);
    }
    println!("{}", squares[4]); // 16

    let offices: HashMap<u16, String> = HashMap::new();
    offices.insert(123, "Jane Smith".to_string());
    offices.insert(196, "Sarah Johnson".to_string());

    println!("{}", offices[&196]);
    println!("{}", offices[&999]);
    println!("{:?}", offices.get(&999));

    let from_c: Complex = (3, 4).into();

    mandelbrot_iters(Complex::new(1, 1));
    mandelbrot_iters((1, 1));
    
    let i: i32 = read();
    let f: f32 = read();
}

// The non-generic toy
fn toy(a: i32, b: i32) -> i32 { // type system is strict, makes it safe.
    a + a + b
}

// The generic toy
//fn add<T: Add>(a: T, b: T) -> T::Output {
//    a + b //
//}

// A generic of type T
fn big_enough<T: shape>(enclosure: &T) -> bool { // returns a bool
    enclosure.area() > 100.0
}

// a method for Option enum
fn get_directions(start: &Location, end: &Location) -> Option<Path> {
    // TODO 
}

// a method to acompancy Result enum
fn ask_user_for_name() -> Result<String, IoError. {
    // ...
}

// a method for From / Into
fn mandelbrot_iters<T>(c: Into<Complex>) -> u32 {
    let c = c.into();
    ...
}

// FromStr example
fn read<T>() -> T
    where T: FromStr,
{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    s.trim.parse().unwrap();
}
