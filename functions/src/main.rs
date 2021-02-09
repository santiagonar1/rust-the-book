fn main() {
    let x = five();
    another_function(x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    // In Rust the last expression is return implicitly. You can determine
    // that is an expression by the fact that does not end with a semicolon.
    // Other option is to use the return keyword.
    5
}
