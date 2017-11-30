extern crate chrono;
use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(date: DateTime<Utc>) -> DateTime<Utc> {
    // for more information about exponentiation in rust https://doc.rust-lang.org/nightly/std/?search=pow
    let x: i64 = 10;
    let temp_result = x.pow64(9));
    assert_eq!(temp_result, 1_000_000_000); // no errors? I guess it's a match
    assert_eq!(temp_result, 1_000_000_000, "Testing if temp_result == 1_000_000_000", a, b); // same, no errors.
    date + Duration::seconds(x.pow64(9)) // calculate and return result;
}
