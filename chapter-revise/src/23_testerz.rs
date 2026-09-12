enum Coin {
   Pnney,
   Shilling,
   Nickel,
   Dime,
   Quarter,
  
}

fn main() {

   fn matchcoin (coin: Coin) -> u8 {
      match coin { 
       Coin::Penny => {
       println!("This is Penney!");
       1
      }
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
    } 
   }

   println!("This Coin is {}", matchcoin(Coin::Penny));

}
