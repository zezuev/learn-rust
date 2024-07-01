#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// all methods defined within an impl block are called associated functions
// each struct is allowed to have multiple impl blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }

    // we can define associated functions that don't have self as their first parameter
    // these are often used for constructors
    fn square(size: u32) -> Self {
        Self {
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

    println!(
        "The area of the rectangle is {} square pixels.",
        // because of automatic dereferencing, we don't need to call (&rect1).area()
        rect1.area(),
    );

    let rect2 = Rectangle {
        width: 25,
        height: 15,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let sq = Rectangle::square(3);
}
