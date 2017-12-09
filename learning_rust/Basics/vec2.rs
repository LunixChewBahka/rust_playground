// vec2.rs
fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

fn main() {
    let mut v = Vec::new();
    v.push(100);
    v.push(200);
    v.push(300);

    dump(&v);

    let slice = &v[1..];
    println!("slice is {:?}", slice);
}
