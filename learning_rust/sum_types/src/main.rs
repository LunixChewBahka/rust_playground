fn main() {
    // as_rest(&self) -> Option<&T>
    let num_as_str: Option<String> = Some("10".to_string());
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `num_as_str` on the stack.
    let num_as_int: Option<usize> =
        num_as_str.as_ref().map(|n| n.len());
    println!("still can print num_as_str: {:?}", num_as_str); // nice!!
    
    // fn as_mut(&mut self) -> Option<&mut T>
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {},
    }
    assert_eq!(x, Some(42));

    // fn expect(self, msg: &str) -> T
    let xe = Some("value");
    assert_eq!(xe.expect("the world is ending"), "value");

    let xen: Option<&str> = None;
    xen.expect("the world is ending"); // panics!?

    // fn unwrap(self) -> T
    // Moves the value v our of the Option<T> if it is Some(v)
    let ux = Some("air");
    assert_eq!(ux.unwrap(), "air");

    let nux: Option<&str> = None;
    assert_eq!(nux.unwrap(), "air"); // this fails?


    // fn unwrap_or(self, def: T) -> T
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    // fn unwrap_or_else<F>(self, f: F) -> T
    // where F: FnOnce() -> T
    let k = 10;
    assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
    assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
}
