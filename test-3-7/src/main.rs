use std::io;
fn main(){


    let a  = [1,2,3,4,5];
    for i in 0..5 {
        println!("This is the position value, {}!",a[i]);
    }
    println!(" This is the 1st position value, {}!", a[0]);  

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed in taking input");
    let index: usize = index.trim().parse().expect("The index shoul be a number");
    let element = a [index];
    println!("the element at index {} is {}", index, element);
} 