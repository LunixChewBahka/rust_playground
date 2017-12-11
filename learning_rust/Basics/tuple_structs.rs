/*
   Tuple Structs

   In tuple structs, fields have types but not names. A programa accesses the fields of a struct by their indices using . operator, e.g., .0;

*/
struct Apple(i32, f64);

// Unit-like structs are structs with no field.
struct Nothing {}

fn main() {
    let fuji = Apple(1, 1.2);
    // we access tuple struct objects by indexing (just like how we would access array elements)
    assert_eq!(fuji.0, 1);
    assert_eq!(fuji.1, 1.2);

    let x = Nothing{};
    // `x` is moved into `y`
    let y = x;
    // cannot use `x` anymore.
    println!("{}", y);
}
