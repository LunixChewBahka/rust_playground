// diverging function - prevents loops

fn main() {
    println!("This Code Runs");
    a_function_that_never_returns();
    println!("This Code Will Never Run"); // this will never be executed
}

fn a_function_that_never_returns() -> ! {
    panic!("The main function will never run code again.");
}
