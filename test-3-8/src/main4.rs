use std::io;
fn main(){

   let mut x1 = String::from("Triumph400X");
   println!("the value of the product is {x1}");
   let r1 = &mut x1;
   r1.insert(x1.len(-1), 's');
   println!("This is r1 after insert {r1}");

}
