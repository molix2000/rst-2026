use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

   let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
   println!("The secert_number is {}",secret_number);

   println!("Creating the user guess input");
   let mut guess: String = String::new();
   io::stdin().read_line(&mut guess).expect("This input was an error");

   println!("The vlue of Guess is {}",guess);
   let guess: u32 = guess.trim().parse().expect("thi input was not a numer");
   match guess.cmp(&secret_number) {
         Ordering::Greater => println!("Too big"),
         Ordering::Less => prinln!("this is too small"),
         Ordering::Equal => println!("This was just right!"),
   }
}  
