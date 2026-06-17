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
    
    println!(" The beginning of the new chapter, enter guess");
    
    let mut guess = String::new();
    let mut guess2 = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Looks like there is an issue with the input format");
    println!("The guess you enterred is {guess}");
    
    println!("Enter Guess2");
    io::stdin()
      .read_line(&mut guess2) 
      .expect("There was an error with the input");
    println!("Guess one was {guess}, Guess2 was {guess2}");
    

    let mut guess03 = String::new();
    io::stdin().read_line(&mut guess03).expect("The inout has an error");
    println!("The guess03 is {guess03}");

    println!("Guess numbers");
    let x = 43;
    let y = 33;
    println!("let x + y = {}, let x - y = {}", x+y, x - y);
    
    let banner_1 = String::from("This is a Rust revised section");
    println!("The target string banner_1 is {}", banner_1);


}
