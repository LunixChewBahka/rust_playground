fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}

fn borrow1(v: &Vec<i32>) {
    println!("{}", (*v)[10] + (*v)[12]); // you are pointing to the actual value
}

fn borrow2(v: &Vec<i32>) {
    println!("{}", v[10] + v[11]); // more idiomatic
}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}

fn main() {
    let mut v = Vec::new();;
    let long_v = vec![4, 5, 6, 7, 8, 9, 2, 3, 4, 2, 4, 3, 2, 3, 3, 5];

    for &i in &long_v {
        let r = count(&long_v, i);
        println!("{} is repeated {} times", i, r);
    }
    
    for i in 1..1000 {
        v.push(i);
    }

    v = re(v);
    println!("Still own v: {} {}", v[0], v[1]);
    
    borrow1(&v);
    println!("Still own v: {} {}", v[0], v[1]);

    borrow2(&v);
    println!("Still own v: {} {}", v[0], v[1]);
}
