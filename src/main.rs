#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        height: 50,
        width: 50,
    };
    let rect2: Rectangle = Rectangle {
        height: 49,
        width: 49,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    let sq1: Rectangle = Rectangle::square(10);
    println!("Area of the square: {}", sq1.area());
}
