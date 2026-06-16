use std::io;

fn main() {
    println!("Revise consepts!");
    println!("Guess a number!");
    
    // This now is the stage of taking in details:

    let mut guess_number = String::new();
    // Use the crate/lib to achieve this result.
    io::stdin()
        .read_line(&mut guess_number).expect("Failed to read a reasonable input");
    println!("You have guessed the number as {}", guess_number);


}
