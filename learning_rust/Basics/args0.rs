// args0.rs
use std::env;

fn main() {
    //for arg in env::args() {
    //    println!("'{}'", arg);
    //}

    // supply arguments after the binary executable (compiled file)
    // ./args0 42 'hello dolly' frodo

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        println!("{:?}", args); // will return a vec
    }
}
