#[derive(Debug)]
struct Rectangle {
    height: i32,
    width: i32
}

fn main() {
    let w: i32 = 1920;
    let h: i32 = 1080;

    let rect: Rectangle = Rectangle {
        height: dbg!(h),
        width: w
    };
    
    let a: i32 = area(&rect);
    println!("Area of a {w} by {h} rectangle: {a}");
    dbg!(&rect);
    
}

fn area(input: &Rectangle) -> i32 {
    input.width * input.height
}