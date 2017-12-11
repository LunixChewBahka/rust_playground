use std::fmt;

#[derive(Debug)]
struct Object { // Type of Object initalization
    width: u32,
    height: u32,
}

// impl is wrapper for methods
impl Object {
    // bind these objects to our Type struct
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

// can also be done for functions
impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object {
            width,
            height,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) and Area: {}", self.width, self.height, self.area())
    }
}

fn main() {
    let o = Object {
        width: 35,
        height: 55
    };

    let obj = Object::new(57, 83);

    o.show();
    println!("{}", o);

    obj.show();
    println!("{}", obj);
}
