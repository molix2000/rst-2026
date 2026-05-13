#[derive(Debug)]

struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn cando(&self, others: &Rectangle) -> bool {
        self.height > others.height && self.width > others.width
    }
}

fn main() {
    let rec1 = Rectangle {
        height: 45,
        width: 09,
    };

    let rec2 = Rectangle {
        height: 43,
        width: 49,
    };

    println!(
        "The outcome of comparing rec1 and rec2 via can do is {}",
        rec1.cando(&rec2)
    );
}
