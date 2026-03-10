use std::io;

fn main() {
    println!("String manipulation in Rust!");
    let a = [1,2,3,4,5];
    println!("Array: {:?}", a);
    println!("Please enter an array index");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("This is wrong type of input");
    
    let index: usize = index.trim().parse().expect("This is the wrong type of input");

    let element = a[index];
    println!(" The value of element at index {index} , element {element}");
    
}
