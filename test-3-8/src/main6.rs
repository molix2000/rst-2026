use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This is an ordering example");
    println!("Guess a number");
    let secret_number = rand::tread_rng()::gen_range(1..=100);

    loop {
        println!("Please input a number");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("The input value was at error");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("This is less"),
            Ordering::Greater => println!("This is more"),
            Ordering::Equal => {
                println!("This is correct");
                break;
            }
        }
    }
}
