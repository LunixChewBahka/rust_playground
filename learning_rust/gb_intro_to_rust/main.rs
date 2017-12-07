fn main() {
    // Doing arithmetic
    // Adding integer numbers
    print!("The sum is {}.\n", 80 + 34);            // The sum is 114.
    print!("{} + {} = {}\n", 34, 80, 80 + 34);      // 34 + 80 = 114
    // 17 % + 20 * 30 / 7
    // 2 + 600 / 7
    // 2 + 85
    // 87
    println!("{}", (23-6) % 5 + 20 * 30 / (3+4));
    
    // Floating point arithmetic
    println!("The sum is {}.", 80.3 + 34.9);        // 115.2
    println!("{}", (23. - 6.) % 5. + 20. * 30. / (3. + 4.)); // 87.something......

    // sequences of statements
    print!("{} + ", 80);
    print!("{} =", 34);
    print!(" {}\n", 80 + 34);

    println!("{}", "These
        are
        four lines
        wtf!");

    println!("{}", "No \
        More \
        Line splitting \
        action :(");

    // three lines with no leading white-space
    println!("{}", "These\n\
        are\n\
        three lines. again");

    /*

    */
}
