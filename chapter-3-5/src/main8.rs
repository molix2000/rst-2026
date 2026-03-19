fn main(){
   let mut count = 0 ;
   'loop_up : loop {
    println!("The count is {count}");
    let mut remain = 10;
    loop {
         println!("Remain is {remain}");
         if remain == 9 {
            break;
         }
         if count == 2 {
            break 'loop_up;
         }
         remain -= 1;
    } //end inner loop
    count += 1;
   } // loop up end
   println!("End count is {count}");




} // main end here
