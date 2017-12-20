// serves as a shortcut, no need to type NumberOrNothing all over again
use self::NumberOrNothing::{Number,Nothing};

enum NumberOrNothing {
    Number(i32),
    Nothing
}

enum Whatever {
    Name(String),
    Num(i64),
}

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    let mut min = Nothing;

    for el in vec {
        match min {
            Nothing => { min = Number(el); },
            Number(n) => { let new_min = min_i32(n, el); min = Number(new_min); }
        }
    }

    return min;
}

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn read_vec() -> Vec<i32> {
    vec![99,234,222,444,555,111]
}

fn print_number_or_nothing(n: NumberOrNothing) {
    match n {
        Nothing => println!("The numebr is: <nothing>"),
        Number(n) => println!("The number is: {}", n),
    };
}

fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_number_or_nothing(min);
}
