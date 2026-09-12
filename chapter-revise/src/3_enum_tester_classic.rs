#[derive(Debug)]

enum UsState {
   Alabama,
   Nibraska,

}

#[derive(Debug)]
enum Coin {

   Penny,
   Nickle,
   Dime,
   Quarter(UsState)
}


fn value_in_cents(coin: Coin) -> u8 {
   
     match coin {
          Coin::Penny => 1,
          Coin::Nickle => 5,
          Coin::Dime => 10,
          Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
          }    
          
    
    }

}

fn main (){
   println!("This is the function value_in_cents in action");
   println!("This is the Penny: {}", value_in_cents(Coin::Penny));
   println!("This is the Nickle: {}", value_in_cents(Coin::Nickle));
   println!("This is the Dime: {}", value_in_cents(Coin::Dime));
   println!("This is the Quarter: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
}
