trait Area {
    fn area(&self) -> i32;
    fn new() -> Self;
}

struct Rectangle {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Area for Rectangle {
    fn area(&self) -> i32 {
        self.w * self.h
    }

    fn new() -> Self {
        Rectangle{ x: 0, y: 0, w: 1, h: 1 }
    }
}

struct Square {
    x: i32,
    y: i32,
    w: i32,
}

impl Area for Square {
    fn area(&self) -> i32  {
        self.w * self.w
    }

    fn new() -> Self {
        Square{ x: 0, y: 0, w: 1 }
    }
}

// Error: `T` is not guaranteed to have the `area()` method.
// error[E0599]: no method named `area` found for type `T` in the current scope
fn print_area<T: Area>(x: T) {
    println!("{}", x.area());
}

// OK: `T` is bounded by `Area`
fn new_area<T: Area>() -> T {
    // Since 'new()' doesn't take any value, we must call 'new()' directly.
    T::new()
}

 
fn main() {
    let x: Rectangle = new_area();
    let y = new_area::<Square>();

    print_area(x);
    print_area(y);
}
