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

    let mut owned_string = String::from("hello ".to_owned());
    let borrowed_string: &str = "world";

    owned_string.push_str(borrowed_string);
    println!("{}", owned_string);

    let msg = "hello world".to_string();
    print_me(msg);

    let answer = 42;
    println!("Hello {}", answer);
}


fn print_me(msg: String) {
    println!("the message is {}", msg);
}
