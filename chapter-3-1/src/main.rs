fn main() {
    println!("Chapter-03-01!");
    let x = 5;
    let y = 10;
    println!("X is {x}, Y is {y}");
    let space_one = "     ";
    let space_one = space_one.len();
    println!("The space_one length is {space_one}");
    let guess : u32 = "42".parse().expect("there as an error");
    println!("Guess is {guess}");
}
