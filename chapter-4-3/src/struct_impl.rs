#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let rec1 = Rectangle {
        height: 46,
        width: 28,
    };

    let rec2 = Rectangle {
        height: 56,
        width: 32,
    };

    let esq = Rectangle::square(3);
    println!("esq value is {:?}", esq);
    println!("The can_hold is with result {}", rec1.can_hold(&rec2));
    println!("The area of rec1 is {}", rec1.area());
}
