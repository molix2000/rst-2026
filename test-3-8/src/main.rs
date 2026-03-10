use std::io;

fn main() {
    println!("String manipulation in Rust!");
    let a = [1,2,3,4,5];
    println!("Array: {0:?}", a);
    println!("Please enter an array index");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("This is wrong type of input");
    
    let index: usize = index.trim().parse().expect("This is the wrong type of input");
    if index >= a.len() {
        println!("Index out of bounds");
        return;
    }
    let element = a[index];
    println!(" The value of element at index {index} , element {element}");

    let mut x1 = String::from("Triumph400X");
    let r1 = &mut x1;
     r1.insert_str(0, "The");
    println!("The value of r1 is {r1}");
}
