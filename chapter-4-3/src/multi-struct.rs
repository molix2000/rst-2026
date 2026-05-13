#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

   fn areaz(&self) -> u32 {
      self.height * self.width
   }

  fn areaz2 (&self) -> u32 {
      self.height * self.width
  } 

  fn areaz3 (&mut self) -> u32 {
     self.height * self.width
  }

  fn areaz4 (self) -> u32 {
     self.height * self.width
  }

  fn can_hold(&self, other: &Rectangle)-> bool {
     self.height > other.height && self.width > other.width
  
  }

}

fn main() {
    let scale = 3;
    let rec1 = Rectangle {
        width: 32,
        height: 55,
    };

    let rec2 = Rectangle {
        width: 54,
        height: 66,
    };

    let rec3 = Rectangle {
        width: dbg!(scale * 33),
        height: dbg!(scale * 44),
    };

    let mut rec4 = Rectangle {
        width: 31,
        height: 38,

    };

    println!("rec1 is , {:#?}", rec1);
    println!("rec2 is {:?}", rec2);
    println!("rec3 is {:#?}", rec3);

    println!("Area for rect 3 is {}",area(rec3.height,rec3.width));
    println!("The rec 3 with area_51 is {}", area_51((rec3.height,rec3.width)));
    println!("The ara_52 result is {}", ara_52((rec1.height,rec1.width)));
    println!("Areaz for rect1 is {}", rec1.areaz());
    println!("Areaz2 output with rec2  is {}",rec2.areaz2());
    println!("Areaz3 output , with rec4 is {}", rec4.areaz3());
    println!("Areaz4 output , with rec3 is {}", rec3.areaz4());
    println!("The comparison result for the can_hold is {}",rec1.can_hold(&rec2)); 
}

fn area(width:u32, height: u32)-> u32{
   width * height
}

fn area_51(dimentions: (u32, u32)) -> u32 {

   dimentions.0 * dimentions.1
}

fn ara_52 (dimentions: (u32,u32)) -> u32 {
   dimentions.0 * dimentions.1
}
