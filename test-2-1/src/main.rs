use std::io;

fn main() {
    println!("Guess 2 Numbers input");
    let mut guess = String::new();
    let mut guess2 = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read iput");
    println!("The input was {guess} ");

    io::stdin()
        .read_line(&mut guess2)
        .expect("Failed to read input");
    println!("the input was {guess2}");


}
