#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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

// Multiple impl blocks are allowed
impl Rectangle {
    fn _print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }
}

fn main() {
    let rect = Rectangle {
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

    println!("{:?}", rect);
    println!("{:#?}", rect);

    dbg!(&rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("{:#?}", sq);
}
