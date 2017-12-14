#[derive(Debug, Clone, Copy)]

enum Shape {
    Circle,
    Triangle,
    Square,
    Pentagon,
    WithSides(usize),
}

fn main() {
    let my_circle = Shape::Circle;
    let my_square = Shape::Square;
    let my_bigogon = Shape::WithSides(100);

    print_number_of_sides(my_bigogon);
}

fn print_number_of_sides(shapes: Shape) {
    use Shape::*; //expose Shape
    println!("{}", match shapes {
        Circle => 1,
        Triangle => 3,
        Square => 4,
        Pentagon => 5,
        WithSides(n) => n,
    });
    // the return type for this is '()'
}
