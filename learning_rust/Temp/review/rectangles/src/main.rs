/**
  5.2 An Example Program Using Structs

  To understand when we might wan to use structs, let's write a program that calculates the area of a rectangle. We'll start with single variables, and then refactor the program until we're using structs instead.

  Let's make a new binary project with Cargo called rectangles that will take the width and height of a rectangle specified in pixels and calculate the area of the rectangle. Listing 5-8 shows a short program with one way of doing just that in our project's src/main.rs:
  */

enum Hero {
    Fast,
    Strong(i32),
    Info {name: String, secret: String}
}

fn get_info(h: Hero) {
    match h {
        Hero::Fast => println!("Fast!"),
        Hero::Strong(i) => println!("Lifts {} tons", i),
        Hero::Info {name, secret} => { println!("{} is {}", name, secret);
        },
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}

fn main() {
    // Area of a circle
    // Let's create a circle
    let mut circle1 = Circle {
        x: 10.0, y: 10.0, radius: 20.0
    };

    println!("{} {} {}", circle1.x, circle1.y, circle1.radius);
    println!("Circle Radius: {}", get_radius(&circle1));
    println!("Circle X: {}", circle1.get_x());
    println!("Circle Area: {}", circle1.area());
    let mut rect1= Rectangle {
        height: 10.0,
        width: 10.0,
    };

    println!("Rect Area: {}", rect1.area());

    let hulk = Hero::Strong(100);
    let quicksilver = Hero::Fast;
    let spiderman = Hero::Info { name: "Spiderman".to_owned(), secret: "Peter Parker".to_owned() };

    get_info(hulk);
    get_info(spiderman);
}

fn get_radius(circle: &Circle) -> f64 {
    circle.radius
}

impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}
