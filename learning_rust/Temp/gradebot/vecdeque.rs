use std::collections::VecDeque;

fn main() {
    let mut vector: VecDeque<u32> = VecDeque::new();

    vector.push_back(0);
    vector.push_back(1);
    vector.push_back(2);

    assert_eq!(vector.as_slices(), (&[0u32, 1, 2] as &[u32], &[] as &[u32]));

    vector.push_front(10);
    vector.push_front(9);

    assert_eq!(vector.as_slices(), (&[9u32, 10] as &[u32], &[0u32, 1, 2] as &[u32]));
}
