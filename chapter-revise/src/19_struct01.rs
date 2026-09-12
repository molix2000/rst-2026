struct QuitMessage;
struct MoveMessage {
  x: i32,
  y: i32,
}


struct Message;
struct WriteMessage(String); // tuple 
                             //
struct ChangeColourMessage(i32,i32,i32);


impl Message {
     fn call (&self){
        let m = String::from("x The Rust training manual");

     }

}

fn main() {
   println!("The Message is {}", Message::call::m());

}




