fn main() {
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {
                println!("X:{:?}", x);
                continue 'outer;
            } // continues the loop over x
            if y % 2 == 0 {
                println!("Y:{:?}", y);
                continue 'inner;
            } // continues the loop over y
        }
    }
}
