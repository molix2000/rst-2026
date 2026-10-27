use std::io;
fn main(){
   println!("Please guess a number");
   println!("type the guess");
   let mutt guess = String::new();
       io::stdin()
         .read_line(&mut guess)
         .expect("Input error);
   println!("The guess is {guess}");
   mainser();

}

fn mainser() {

   println!("mainser fn began now");
   {
     mut guess2 = String::new();
     guess2.pust_str("44");
     println!("The inner part of fun mainser is printing the value of guess2 as {guess2}");
   }

}



