/*

   Problem 1

   If we list all the natural numbers below 10 that are multiples of 3 or 5,
   we get 3, 5, 6 and 9. The sum of these multiples is 23.

   Find the sum of all the multiples of 3 or 5 below 1000.
*/
fn main() {
    // Let us start with 10 first before we go 1000
    let below_ten: u32 = 1000;

    let mut sum_val = 0u32;
    // iterate using for loop 0..1000 
    for value in 0u32..below_ten {
        if value % 3 == 0 || value % 5 == 0 {
            println!("{} is a multiple of 3 or 5.", value);
            sum_val += value;
        } else {
            println!("-");
        }
        //println!("[{}] Hello, world!; Value = {}; sum_val: {}", index, value, sum_val); 
    }
    // answer should be 233,168
    println!("The sum of all the multiples of 3 or 5 is: {:?}", sum_val);
}
