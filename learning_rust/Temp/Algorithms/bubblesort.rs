// Bubble Sort implementation in rust programming

fn main() {
    let v = vec![99, 11, 10, 8, 55, 35, 125, 500, 1];
    println!("{:?}", v);
    println!("{:?}", bubble_sort(&v));
}

fn bubble_sort(vec_to_sort: &Vec<i32>) -> Vec<i32> {
    let mut result = vec_to_sort.clone();
    for i in 0..result.len() {
        for y in 0..result.len() {
            if result[i] < result [y] {
                result.swap(i, y);
            }
        }
    }
    return result;
}
