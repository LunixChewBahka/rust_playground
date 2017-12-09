// array2.rs
// read as : slice of i32
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn main() {
    let arr = [100, 200, 300, 400];
    // look at the '&'
    let res = sum(&arr);
    println!("sum of all the values inside the array: {}", res);
}
