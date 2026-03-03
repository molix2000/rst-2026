###



fn main() {

    let mut guess 


    let guess: u32 = guess.trim().parse.expect("Type a number");
    match  guess.cmp(&secret_number){
           Ordering::Less => println!("This is smaller");
           Ordering::Greater => println!("This is larger");
           Ordering::Equal => println!("This is Correct you Win");
    }


}
