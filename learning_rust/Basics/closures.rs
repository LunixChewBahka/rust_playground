// Getting started with closures
fn main() {
    // let var = |closure arguments| closure operation or do something with args;
    let plus_one = |x: i32| x + 1;
    println!("{}", plus_one(4)); // 5

    let plus_two =  |x| {
        let mut result: i32 = x;

        result += 1;
        result += 1;

        result
    };
    println!("{}", plus_two(4)); // 6

    // no need to annotate types of args the closure takes or the values it returns.
    let num = 5;
    let plus_num = |x: i32| x + num;
    println!("{}", plus_num(5));
    let y = &mut num;

