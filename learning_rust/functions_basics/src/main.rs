fn main() {
    let x = 4;
    //add_another_number(x);
    println!("add_another_number {:?}", add_another_number(x));
    print_sum(3, 4);
    println!("Return int using '-> <var type>' {:?}", print_sum_return_int(5, 25));
    println!("Add one result: {:?}", add_one(99));

    println!("{0} {1} {2} - {1} {0} {2}", "Alice", "Bob", "Jar");
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 511);
}

fn add_another_number(x: i32) -> i32{ // takes and returns an i32 number
    if x < 5 {
        return x;
    } else {
        return x + 1;
    }

}

fn print_sum(a: i32, b: i32) { // this works
    println!("sum is: {}", a + b);
}

fn print_sum_return_int(c: i32, d: i32) -> i32 {
    c + d
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// function that do not return.
// panic! is a marci, siliar to println!, panic! () causes the thread of execution to crash with the given message.
fn diverges() -> ! {
    panic!("This function never returns!");
}
