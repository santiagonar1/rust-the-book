fn main() {
    let condition = true;
    let number = if condition { 4 } else { 6 };

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
