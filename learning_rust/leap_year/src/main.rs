fn main() {
    let x_year = 2400;
    let temp_result = is_leap_year(x_year);
    println!("Is year {:?} a leap year? ans. {:?}", x_year, temp_result);
}

fn is_leap_year(year: i32) -> bool { // fn takes an i32 value and return a bool
    // ON every year that is evenly divisible by 4
    // EXCEPT every year that is evenly divisible by 100
    // UNLESS the year is also evenly divisible by 400
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        true
    } else {
        false
    }
}

