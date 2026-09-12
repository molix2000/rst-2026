use std::*;

#[derive(Debug)]
struct Rectangle {

    height: u32,
    width: u32,

}

#[derive(Debug)]
struct Colour (u8, u8, u8);


#[derive(Debug)]
struct Mystruct;

impl Rectangle {
     fn area_self (&self) -> u32 {
        self.height * self.width
     }
}

#[derive(Debug)]
struct Circle {
      radius: f64,
}

impl Rectangle {
     fn can_hold (&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
     }
}

impl Rectangle {
     fn square(size: u32) -> Rectangle {
         Rectangle { width: size, height: size }
     }
    
}

impl Colour {
     fn red(&self) -> u8 {
        self.0
     }
     
     fn blue(&self) -> u8 {
       self.1
     }

}

impl Mystruct {

   fn printmessage() -> String {
      println!("This is open claw stuff from printmessage before returning a string");
      "Test Claw".to_string()
  }
}


impl Circle {
      fn area(&self) -> f64 {
         std::f64::consts::PI * (self.radius * self.radius)
      }
     // an associated function to create new Circle. This seres as constructor
     fn new(radius: f64) -> Circle {
        Circle {radius} 
     }
}

fn main() {



    let rect1 = Rectangle {
        height: 33,
        width: 21,
    };


    let rect2 = Rectangle {
        height: 11,
        width: 51,  
    };


    let rect3 = Rectangle {
        height: 14,
        width: 10,
    };

    let sq = Rectangle::square(3);
    let circ = Circle::new(10.0);
    let white = Colour (255,255,255);
    let red = Colour (255,0,0);
    println!("This is the rect1 {:#?}", rect1);
    println!("The area for the sturct is {:#?}",rect1.area_self());
    println!("This is the rect2 {:#?}", rect2);
    println!("The area_self for rect2 is {}",rect2.area_self());
    println!("This is the rect3 {:#?}", rect3);
    println!("The area_self for rect3 is {}",rect3.area_self());
    println!("The rectangle square sub-func output is {:#?}", sq);
    println!("The holder for one against two is {}", rect1.can_hold(&rect2));
    println!("The colour white is {:#?}", white);
    println!("The red in red method for tuple {}, but 2nd is {}", red.red(), red.blue());
    println!("This is from Mystruct {}", Mystruct::printmessage());
    println!("There is of the circle is {:.2}", circ.area());
}

