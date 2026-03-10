use std::io;
fn main(){

   let mut x1 = String::from("Triumph400X");
   println!("the value of the product is {x1}");
   const POSIT : u8 = 10;
   let r1 = &mut x1;
   r1.insert(POSIT.into(), 's');
   println!("This is r1 after insert {r1}");

}
