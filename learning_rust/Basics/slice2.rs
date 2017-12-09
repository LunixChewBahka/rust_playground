// slice2.rs
fn main() {
    let ints = [1321, 2, 3, 4, 200];
    let slice = &ints;
    let first = slice.get(0);
    let last = *slice.get(10).unwrap_or(&-1);

    // simply returns the actual value of the element not the index
    println!("first: {:?}", first); 
    // if the value given goes out of bounds; it will just return None
    println!("last: {:?}", last);
    println!("first: {} {}", first.is_some(), first.is_none());
    println!("last: {} {}", last.is_some(), last.is_none());
    println!("first value: {}", first.unwrap());

    let x = &mut [50, 25, 10];
    if let Some(elem) = x.get_mut(2) {
        *elem = 42;
    }
    // from [50, 25, 10], it will change to [50, 25, 42]
    println!("{:?}", x);
}
