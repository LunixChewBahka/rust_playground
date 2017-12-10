pub fn reverse(word: &str) -> String {
    let drow: String = word.chars().rev().collect::<String>();
    drow
}
