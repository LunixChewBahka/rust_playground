// arrays is simply a list of items; immutable (by default) - mutable/changeable/alterable
// mutability vs immutability

fn main() {
    // initialize an array
    let shopping_list = [1, 3, 5, 7, 9, 11, 13]; // let = const... kind of
    let mut shopping_list_mut = [1, 3, 5, 7, 9, 11, 13]; // let = const... kind of
    println!("{:?}", shopping_list_mut);
    shopping_list_mut[2] = 500;
    println!("{:?}", shopping_list_mut);
    shopping_list_mut[5] = 1_000_00;
    println!("{:?}", shopping_list_mut);
}
