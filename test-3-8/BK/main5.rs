use std::io;
fn main () {
   println!("String manupulation");
   let mut a = [1,2,3,4,5];
   let mut index = String::new();
   println!("Please give an index");
   io::stdin().read_line(&mut index).expect("This index is not in the correct type");
   let index: usize = index.trim().parse().expect("This index in not in the tisht format");
   if index >= a.len() {
      println!("Out of range");
      return;
   }
   let element = a[index];
   println!("element is {element}, for index {index}");





}
