

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// First 'impl' block.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

// Second 'impl' block.
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("The area of the rectangle is {} square pixels", rect1.area());
    println!("{:#?}", rect1);

    // can hold function
    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // associated function
    let sq = Rectangle::square(5);

    println!("\nSquare rect: {:#?}", sq);
}