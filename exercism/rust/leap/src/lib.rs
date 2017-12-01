pub fn is_leap_year(year: i32) -> bool {
    // ON every year that is evenly divisible by 4
    // EXCEPT every year that is evenly divisibly by 100
    // UNLESS the year is also evenly divisible by 400

    // brute force way ... gonna look for a more elegant approach
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        true
    } else {
        false
    }
}
