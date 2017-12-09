use std::mem;
// string6.rs
fn array_to_str(arr: &[i32]) -> String {
    // declare res as mutable; start of the string '['
    let mut res = '['.to_string();
    // convert every element found in the array, convert to string append /push 
    // to res and add a ',' after every element
    for v in arr {
        res += &v.to_string();
        res.push_str(", ");
    }
    // extra ',' is popped?
    res.pop();
    res.pop();
    // close out the stringified array with ']'
    res.push(']');
    // return the stringified array
    res
}

fn main() {
    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);

    assert_eq!(res, "hello [10, 20, 30]");
    println!("{} {}", res, arr);

    println!("Before stringified, the array occupies {} bytes", mem::size_of_val(&[10, 20, 30]));
    println!("After stringified, the array occupies {} bytes", mem::size_of_val(&arr));
}
