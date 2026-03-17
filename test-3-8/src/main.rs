use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(" This is the ordering and random number combined");
    let mut guess = String::new();
    println!("Enter a guess number");
    io::stdin().read_line(&mut guess).expect("This entry was in error");

    let  secret_number = rand::thread_rng()
        .gen_range(1..=100);
    println!("The random number is {secret_number}");

    println!("Your guess number is {guess}");

    let guess: u32 = match guess.trim().parse() {
        Ok(num)  => num,
        Err(_) => {
            println!("Error");
            33
        }
    };

    match guess.cmp(&secret_number) {
        Ordering::Greater => println!("This is greater value"),
        Ordering::Less => println!("This is less than"),
        Ordering::Equal =>  println!("This is Equal"),                   
    }

    println!("Coming to an end");
}
