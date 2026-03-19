fn main() {
   println!("Chapter 3.5, loop equals a variable");
   let mut counter = 0;
'counting_ip: loop {
    println!("Count e {counter}");
    let mut remaining = 10;
    loop {
        println!("The remaining times are{remaining}");
        if remaining == 9 {
            break;
        }
        if counter == 2 {
            break 'counting_ip;
        }
        remaining -= 1;
    }   
    counter += 1;
} 
 println!("End of the loop, counter is {counter}");


}