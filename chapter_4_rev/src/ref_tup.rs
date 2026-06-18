//use rand::Rng;
//use std::cmp::Ordering;
use std::io;
fn main(){ 
  let rect1 = (22,32);
  println!("The area is {}", area(rect1));


}

fn area (dimentiones: (u32,u32)) -> u32 {


    dimentiones.0 * dimentiones.1

}
