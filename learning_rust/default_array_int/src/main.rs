fn main() {
    // 1st way of initializing a 4 element array
    let my_array = [5, 5, 5, 5];
    // print the array contents
    println!("{:?}", my_array);

    // 2nd way of initializing
    let my_array_two = [100;4]; // initialize 4 times '100' inside the array
    // print 2nd way
    println!("{:?}", my_array_two);

}
