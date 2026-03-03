fn main() {
    println!("Number/ Input Guess");
    
    let mut guess = String::new();
    println!("Input number");
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to obtain input");
    println!("The input was {guess}");

}
