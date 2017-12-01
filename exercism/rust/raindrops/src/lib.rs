pub fn raindrops(n: usize) -> String {
    check_divisibility(n)
}

// brute force what the f!
// need to find a shorter, more elegant way
// to get around this problem
// will try match statement for the next iteration
pub fn check_divisibility(n: usize) -> String {
    // receives a num and returns a value 
    // without a type (tuple by default)
    if n % 3 == 0 && n % 5 == 0 && n % 7 == 0 {
        return "PlingPlangPlong".to_string();
    } else if n % 3 == 0 && n % 5 == 0 {
        return "PlingPlang".to_string();
    } else if n % 3 == 0 && n % 7 == 0 {
        return "PlingPlong".to_string();
    } else if n % 5 == 0 && n % 7 == 0 {
        return "PlangPlong".to_string();
    } else if n % 7 == 0 {
        "Plong".to_string()
    } else if n % 5 == 0 {
        "Plang".to_string()
    } else if n % 3 == 0 {
        "Pling".to_string()
    } else {
        n.to_string()
    }
}
