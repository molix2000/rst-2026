fn main() {
    println!("Process input");
    let mut guess = String::new();
    println!("Input a number");
    std::io::stdin().read_line(&mut guess).expect("Failed to read any input");
    println!("The Input was {guess}");

    println!("Input a second string");
    let mut student = String::new();
    println!("Input new sudent name here");
    std::io::stdin().read_line(&mut student).expect("Failed to catch input");
    println!("The input was {student}"); 
}

