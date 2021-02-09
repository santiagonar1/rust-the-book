const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6_000; // We can use _ as separator for clarity
    println!("The value of x is: {}", x);
    println!("The constant is: {}", MAX_POINTS);

    // Tuple declaration
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuple: {}, {}, {}", x, y, z);
    println!("Tuple: {}, {}, {}", tup.0, tup.1, tup.2);
}
