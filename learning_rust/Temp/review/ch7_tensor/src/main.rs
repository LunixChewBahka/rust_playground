use std::collections::HashMap;
use std::fs::File;

#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Text(String),
}

fn main() {

    let fl = File::open("test.txt");

    let fl = match fl {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        },
    };

    // pointer to the data, length, capacity
    // capacity indicated how much memory is reserved for the vector
    // the vector will grow so long as the length < capacity
    let x = vec![1, 2, 3, 4];
    let mut v: Vec<i32> = Vec::new();

    // can be useful for various types of programs
    let r = vec![
        Example::Int(142),
        Example::Float(12.32),
        Example::Text(String::from("somestring")),
    ];

    println!("{:?}", &r);

    for i in &v {
        println!("{}", i);
    }

    println!("Vector: {:?}, Length: {}, Capacity: {}", &v, v.len(), v.capacity());
    println!("{:?} was popped!", v.pop());

    let mut hm = HashMap::new();

    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 49);
    hm.remove(&String::from("strings"));

    for (k, v) in &hm {
        println!("{}: {}", k, v);
    }

    match hm.get(&String::from("no key")) {
        Some(&n) => println!("{}", n),
        _ => println!("no match"),
    }

    let mut s = Some(0);

    while let Some(i) = s {
        if i > 19 {
            println!("Quit");
            s = None;
        } else {
            println!("{}", i);
            s = Some(i + 2);
        }
    }
}
