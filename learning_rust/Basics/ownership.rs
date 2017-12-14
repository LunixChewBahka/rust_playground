// ownership
// The '&' sigil means borrowed reference
// Borrowing prevents objects from moving around
use std::rc::Rc;

struct MyStruct { inner: i32 }

struct A { b: B }
struct B { c: Box<i32> }

fn get(s: &MyStruct) -> &i32 {
    &s.inner
}

fn work_with(data: Rc<int>) {}

fn main() {
    let data = Rc::new(3i32);
    {
        let data2 = data.clone();
        work_with(data2);
    }
    work_with(data);
    let my_three = helper();
    println!("my_three {}", my_three);

    let a: A = A {
        b: B {
            c: Box::new(2)
        }
    };

    let c = a.b.c;

    //let b: B = a.b; // error (move by `c`)
    println!("{}", c);

    let s = MyStruct { inner: 3 };
    let inner = get(&s); // same lifetime as `s`
    println!("{}", inner);
}

fn helper() -> Box<i32> {
    let three = Box::new(3);
    return three;
}
