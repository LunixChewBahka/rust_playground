fn main() {
    // scalar value = single value(char);
    // string =  a sequence of characters; type of utf-8 char set
    /*
       2 types of string in rust:
            1. reference string (string slices in rust)
                    - has a fixed sized therefore cannot be changed;
            2. 
    */
    let hello_world = "Hello from the other side.";
    println!("{:?}", hello_world);
}
