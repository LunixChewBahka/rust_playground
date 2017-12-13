// Bubble Sort implementation in rust programming
fn main() {
    let mut arr = [9, 81, 11, 7, 9, 0, 27, 55, 69];
    println!("This is {:?}", arr);
    change_value(&mut arr);
    println!("This is {:?}", arr);
}

fn change_value(arr: &mut [i32]) {
    arr[4] = 88
}
