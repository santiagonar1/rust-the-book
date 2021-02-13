#[derive(Debug)] // this annotation allows to print with :?
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function (i.e., not 'self' argument)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(20);

    println!("rect1 is {:?}", rect1);
    println!("rect2 is {:?}", rect2);
    println!("rect3 is {:?}", rect3);
    println!("square is {:?}", square);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
