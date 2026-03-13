use std::io;

fn main() {

   println!("The string/array processor");

   let a = [1,2,3,4,5,6];
   println!("This is the array {:?}",a);
   println!("Please input an index guess");
   let mut index = String::new();
   io::stdin().read_line(&mut index).expect("There is was an error processing this input");
   let index: usize = index.trim().parse().expect("There was an error processing input");
   let element = a[index];
   println!("The element within the string is {element}, the index was {index}");



}
