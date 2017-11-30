use std::io;

fn main() {

    println!("Enter an integer from 1-20: ");

    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read line.");

    let number: u32 = number.trim().parse()
        .expect("Please type a number!");

    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime."),
        // Match an inclusive range
        13...19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special."),
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms ofa match must cover all the possible values
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
