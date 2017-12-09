// iter2.rs
fn main() {
    let arr = [10, 20, 30];
    // with arrays you need add iter() explicitly
    for i in arr.iter() {
        println!("{}", i);
    }

    // slices on the other hand are converted implicitly to iterators..
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }

    // it is more efficient to iterate over an array or slice this way than
    // to use "for i in 0..slice.len() {}" because Rust does not have to 
    // obsessively check every index operation.
}
