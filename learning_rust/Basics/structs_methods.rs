/*
   A structure contains a sequence of data, called "fields".
*/

struct Apple {
    color: i32,
    weight: f64,
}

fn main() {
    let fuji = Apple { color: 1, weight: 1.2 }; // feed data to struct;
    // `golden` takes the value for the `weight` field from `fuji`.
    let golden = Apple { color: .. fuji, weight: 3.333 };
    println!("{} {}", fuji.color, fuji.weight);
    println!("{} {}", golden.color, golden.weight);
}
