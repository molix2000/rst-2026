#[derive(Debug)]
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,

}

fn value_in_cents(coin: Coin) -> u8 {
   match coin {
       Coin::Penny => 1,
       Coin::Nickel => 5,
       Coin::Dime => 10,
       Coin::Quarter => 25,

   }
  
 
}


fn main(){

   println!("The Penny: {:#?} ", Coin::Penny);
   println!("The Nickel: {:?}", Coin::Nickel);

}
