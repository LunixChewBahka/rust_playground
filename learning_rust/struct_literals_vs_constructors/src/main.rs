// a review of structs in rust
struct Point {
    x: i32,
    y: i32,
}

mod foo {
    pub struct Point {
        x: i32,
        y: i32,
        _secret: (),
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Point {
            Point { x: x, y: y, _secret: () }
        }
    }

    pub use foo::point::Point;

    fn foo() -> Point {
        Point::new(0, 0);
    }
}

fn main() {
    // we can create 'struct literal syntax' to create a new instance of the struct:
    //let origin = Point { x: 0, y: 0 };
    //println!("Point ({}, {})", origin.x, origin.y);

    let origin2 = foo::Point::new(0, 0);
    //println!("{:?}", origin2);
}
