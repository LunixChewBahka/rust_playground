fn main() {
    let my_tuple = ('k', 45);
    println!("Our tuple contains the letter: {:?}", my_tuple.0);
    println!("Our tuple contains the number: {:?}", my_tuple.1);

    // tuple destructuring in rust e.g. : (435, 'f', another_object)
    // what if we want to store each value in a variable? ans.tuple destructuring sytax
    let (a,b) = my_tuple; // declared two separate variables for destructuring
    println!("Fancy: {:?}", a);
    println!("Pancy: {:?}", b);
}
