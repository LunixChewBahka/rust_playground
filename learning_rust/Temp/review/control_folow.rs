fn foo(x: i32) -> &'static str {
    let result: &'static str;
    if x < 10 {
        result = "less than 10";
    } else {
        result = "10 or more";
    }
    return result;
}

fn bar(x: i32) -> &'static str {
    if x < 10 {
        "less than 10"
    } else {
        "10 or more"
    }
}

fn main() {
    println!("{}", foo(10));
    println!("{}", bar(3));

    let mut x = 10;
    while x > 0 {
        println!("Current value: {}", x);
        x -= 1; // basically x = x-1
    }

    let names = vec!["Jane", "Jill", "Jack", "John"];
    let total_bytes = names
        .iter()
        .map(|name: &&str| name.len())
        .fold(0, |acc, len| acc + len);

    println!("{} bytes", total_bytes); // 16 is expected

    let values = vec![1, 2, 3, 4, 5];
    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                let next;                                  
                match iter.next() {
                    Some(val) => next = val,
                    None => break,
                };
                let x = next;
                let () = { println!("{}", x) };
            },
        };
        result
    }

    let lazy_v: Vec<i32> = vec![2, 4, 6, 8, 10].into_iter().map(|x| x + 1).collect();
    println!("{:?}", lazy_v);
    let mut lazy_c = 0;
    for pair in vec!['a', 'b', 'c'].into_iter()
        .map(|letter| { lazy_c += 1; (letter, lazy_c) }) {
            println!("{:?}", pair);
        }

    let mut lazy_c2 = 0;
    for pair in vec!['a', 'b', 'c'].into_iter()
        .map(|letter| { lazy_c2 += 1; (letter, lazy_c2) })
            .rev() {
                println!("{:?}", pair);
            }
}
