use std::collections::VecDeque;

fn main() {
    let a = [1, 2, 3];
    let doubled: Vec<i32> = a.iter()
        .map(|&x| x * 2)
        .collect::<Vec<_>>();

    println!("{:?}", doubled);

    let charst = ['g', 'd', 'k', 'k', 'n'];
    let hello: String = charst.iter()
        .map(|&x| x as u8)
        .map(|x| (x + 1) as char)
        .collect();
    println!("{:?}", hello);

    let mut vector: VecDeque<u32> = VecDeque::new();

    vector.push_back(0);
    vector.push_back(1);
    vector.push_back(2);

    assert_eq!(vector.as_slices(), (&[0u32, 1, 2] as &[u32], &[] as &[u32]));

    vector.push_front(10);
    vector.push_front(9);

    assert_eq!(vector.as_slices(), (&[9u32, 10] as &[u32], &[0u32, 1, 2] as &[u32]));
}
