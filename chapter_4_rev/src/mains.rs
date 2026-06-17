use std::io;

fn main() {

   let mut arla = String::new();
   println!("Input a string");
   io::stdin().read_line(&mut arla).expect("There was an error receiving the input");
   let index: usize = arla.trim().parse().expect("There is an error with the i parse inspect");
   println!("The index is {}",index);

}


