struct Equal_Band {
    name: String,
    address: String,
}

struct Rectangle {
   width : u32,
   height : u32,
}

pub fn main() {
    let eb = Equal_Band {
        name: String::from("Test Subject A"),
        address: String::from("299 harley cresent, fedora avenue, rockford"),
    };

    println!("eb name is {}", eb.name);
    println!("The eb Address is {}", eb.address);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(" The area of the rectangle is {}", area(rect.width, rect.height));
} 

pub fn area(width: u32, height: u32) -> u32 {
    width * height
}

