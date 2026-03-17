use std::io;

fn main() {
    println!("Please give an integer number");
    let mut inputer = String::new();
    io::stdin()
        .read_line(&mut inputer)
        .expect("There has been an error taking your input");
    println!("The number given is {inputer}");
}
