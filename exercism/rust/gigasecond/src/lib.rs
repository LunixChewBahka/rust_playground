extern crate chrono;
use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(date: DateTime<Utc>) -> DateTime<Utc> {
    // for more information about exponentiation in rust https://doc.rust-lang.org/nightly/std/?search=pow
    let x: i64 = 10; // x takes a value of 10 which is of type i64 (64-bit integer)
    let giga_sec_val = x.pow(9); // 10 raise to the power of 9
    assert_eq!(giga_sec_val, 1_000_000_000); // no errors? I guess it's a match
    assert_eq!(giga_sec_val, 1_000_000_000, "Testing if temp_result == 1_000_000_000"); // same, no errors.
    date + Duration::seconds(giga_sec_val) // calculate date + duration which is converted to seconds and return result;
}
