struct  Equal_Band {
  name: String,
  address: String,
  }

pub fn struct02() {
    let eb = Equal_Band {
      name: String::from("Test Subject A"),
      address: String::from("299 harley cresent, fedora avenue, rockford"),
    };

  println!("eb name is {}", eb.name);
  println!("The eb Address is {}", eb.address);
}

fn main() {
   let width = 30;
  let height = 50;

  println!(" The area of the rectangle is {}", area(width , height));
} 

fn area(width: u32, height: u32) -> u32 {
    width * height
}

