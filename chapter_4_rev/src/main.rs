use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

   let mut arla = String::new();
   println!("Input a string");
   io::stdin().read_line(&mut arla).expect("There was an error receiving the input");
   let index: usize = arla.trim().parse().expect("There is an error with the i parse inspect");
   println!("The index is {}",index);

   println!("New Chapter here");
  
  let mut guess = String::new();
  println!("Guess a number?");
  io::stdin().read_line(&mut guess).expect("This input was at error");
  let secret_number = rand::thread_rng().gen_range(1..=100);
  println!("Guess secret number");
  let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Error");
        33
     }

  };

  match guess.cmp(&secret_nunmber) {
      Ordering::Greater => println!("This is greater value"),
      Ordering::Less => println!("This is a less value"),
      Ordering::Equal => println!("This is great"),

  }

}


