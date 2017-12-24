/*
   Problem 01: Vector & Slice Manipulaiton

   Vectors, iteration, pass-by-ref, mutability, function pointers.

   Create a new module in your library named problem1 insidie problem1.rs

   to get a first taste of basic Rust, complete the following three functions. Not that all these functions take their arguments by reference, rather than by value.

   Don't use any of the standard library methods on the Vec class which implement the target behavior, since using them would defeat the point of this exercise :) (However, basic functions such as contains() and push() are fine.
   */


/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    unimplemented!();
}

/// Defuplicates items in the input vector `vs`. Produces a vector containing the
/// first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    // TODO
    unimplemented!();
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    // TODO
    unimplemented!();
}
