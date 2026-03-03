use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

   
   println!("Input a guress number ");
   let mut guess = String::new();
   
   io::stdin()
      .read_line(&mut guess).expect("There is a read error");
   println!("The guess is {guess}");

   let guess: u32 = guess.trim().parse().expect("Please type number");

   let secret_number = rand::thread_rng().gen_range(1..=100);
   println!("The random number is {secret_number}");

   match guess.cmp(&secret_number){
      Ordering::Less => println!("This is smaller"),
      Ordering::Greater => println!("This is larger"),
      Ordering::Equal => {
         println!("This is correct and matching");
         break;
   }    
}
