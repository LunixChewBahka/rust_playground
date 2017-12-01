fn main() {
    let mut x: Vec<i32> = Vec::new();
    x.push(4);
    x.push(7);
    x.push(69);
    x.push(69);
    x.push(69);
    x.push(69);
    x.push(69);
    println!("{:?}", x.capacity());

    let mut m_vec: Vec<i32> = Vec::with_capacity(5); // 5?
    m_vec.push(5);
    m_vec.push(6);
    m_vec.push(8);
    m_vec.push(2);
    m_vec.push(4);
    m_vec.push(9);
    println!("{:?}", m_vec.capacity()); // automatically or dynamically resize themselves
}
