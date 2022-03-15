#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    // 1.
    // let length1 = 50;
    // let width1 = 30;

    // 2.
    // let rect1 = (50, 30);

    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("rect1 is {:?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// 1.
// fn area(length: u32, width: u32) -> u32 {
//     length * width
// }

// 2.
// Use tuple
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}