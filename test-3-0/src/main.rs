use std::io;

fn main() {
    
    
    println!("Chapter 3");
    println!("Shadowing");
    let mut x = 5;
    println!("The value of X is {x}");
     x = 7;
    println!("The value of x this time shadowed is {x}");
    println!("The mut allow shadows");

    
    println!("Constants must be in capital");
    const IN_SECONDS: u32 = 60 * 60 * 30;
    const TITLE: &str = "This is chapter 3";
    println!("This is 3 hours,{IN_SECONDS}");
    println!("Sigma is , {TITLE}");




}
