struct Point {
    x: i32,
    y: i32,
}

fn transform(p: Point) -> Point {
    Point { x: p.x + 1, y: p.y + 1 }
}

fn main() {
    let p0 = Point { x: 5, y: 10 };

    let p1 = transform(p0);

    let s1 = String::from("Jarelim Koi L. Padul");

    let number = 5;
    let succ_number = succ(&number);
    // len takes a function "calculate_length" which in return takes a parameter
    // that refers to s1 (&s1)
   let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    println!("{}", succ_number);

    println!("{:?}", p1);
}

// calculate_length takes 's' which is of Type String
fn calculate_length(s: &String) -> usize {
    // length of 's' is determined by len() then immediately returns not as String
    // but as usize
    // usize is a more general way of giving an Int, UInt, Primitives etc 'of Type'
    // capacity of usize depends on the CPU architecture you are running
    s.len()
}

fn succ(x: &i32) -> i32 { x + 1 }

