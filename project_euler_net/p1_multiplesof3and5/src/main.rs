/*

   Problem 1

   If we list all the natural numbers below 10 that are multiples of 3 or 5,
   we get 3, 5, 6 and 9. The sum of these multiples is 23.

   Find the sum of all the multiples of 3 or 5 below 1000.
*/
fn main() {
    // Let us start with 10 first before we go 1000
    // using collected iterator - which works like a charm :D
    //let mut nums_below_1000: Vec<u32> = (0..10).collect();
    //println!("Collected (0..1000) into: {:?}", nums_below_1000);
    let foo: Vec<u32> = (0..10).collect();
    for (i, item) in foo.iter().enumerate() {

        //println!("Found index [{}]: item is {}", i, item);
    }

    //println!("{}", val_sum);
    // answer should be 233,168
}
