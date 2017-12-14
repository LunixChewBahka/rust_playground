// Traits and Generic
// Traits are very similar to Interface, allows us to write how functions should look like
struct Fib {
    c: u32,
    n: u32,
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;

        Some(self.c)
    }
}

fn fib() -> Fib {
    Fib{c: 1, n:1}
}

fn main() {
    for j in fib().take(10) {
        println!("{}", j);
    }

    for j in fib().skip(14).take(10) {
        println!("{}", j);
    }
}
