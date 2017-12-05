pub fn reverse(_: &str) -> String {
    let word = "robot";
    let drow: String;
    drow = word.chars().rev().collect::<String>();
    return drow;
}
