struct Equal_Band {
    name: String,
    address: String,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn main() {
    let eb = Equal_Band {
        name: String::from("Test Subject A"),
        address: String::from("299 harley cresent, fedora avenue, rockford"),
    };

    println!("eb name is {}", eb.name);
    println!("The eb Address is {}", eb.address);
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: dbg!(50 * scale),
    };
    println!(
        " The area of the rectangle is {}",
        area(rect.width, rect.height)
    );
    dbg!(&rect);
    println!("The Struct is {:#?}",rect);
}

pub fn area(width: u32, height: u32) -> u32 {
    width * height
}
