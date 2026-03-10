use std::io;

fn main() {
    println!("String manipulation in Rust!");
    let a = [1,2,3,4,5];
    println!("Array: {:?}", a);
    
    let mut index = String::new();
    io::stdin().red_line(&mut index).expect(This is wrong type of input");
    
    let index: usize = index/trim().parse.expect("This is the wrong type of input");

    let element = a[index];
    println!(" The value of element at index {index} , element {element}", index,element);
    
}
