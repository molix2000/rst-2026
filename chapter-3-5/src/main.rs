fn main() {
    println!("Chapter 3.5!");
    println!("Handling multiple conditions!");
    let number = 6;
    if number % 4 == 0 {

        println!("Number is diviable by 4");
    } else if number % 3 == 0 {
        println!("The number is divisable by 3");
    } else if number % 2 == 0 {
        println!("the number is divisable by 2");
    }
    println!("number was divisable by a number from the above list:");
    let condition = true;
    let condition2 = false;
    let suggestion = if condition {8} else {0};
    let suggestion2 = if condition2 {9} else {7};
    println!(" suggestion is {suggestion}, suggestion2 is {suggestion2}");
}
