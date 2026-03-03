use std::io;
use rand::Rng;

fn main() {

    println!("This is the randon number generator");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    println!("Give us a guess number");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess).expect("There has been an error in reading");
    println!("The guess was {guess}");


}
