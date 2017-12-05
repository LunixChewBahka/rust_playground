use std::fmt::{self, write};

struct Position {
    longitude: f32,
    latitude: f32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.longitude, self.latitude)
    }
}

fn main() {
    for n in 1..101 {
        match n {
            n if n % 15 == 0 => println!("FizzBuzz"),
            n if n % 5 == 0 => println!("Buzz"),
            n if n % 3 == 0 => println!("Fizz"),
            n => println!("{}", n)
        }
    }

    let mut output = String::new();
    match write(&mut output, format_args!("Hello {}!", "world")) {
        Err(fmt::Error) => panic!("An error occurred"),
        _ => (),
    }
    assert_eq!(output, "Hello eeee world!"); // this will cause panic
    println!("{:?}", output);

    assert_eq!("(1.987, 2.983)".to_owned(),
                format!("{}", Position { longitude: 1.987, latitude: 2.983, }));
    let origin = Position { longitude: 1.987, latitude: 2.983 };
    println!("The origin is: {}", origin);
}
