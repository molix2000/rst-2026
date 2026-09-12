struct QuitMessage; 
struct MoveMessage {

    x: i32,
    y: i32,

}



impl MoveMessage {
     fn call (&self) {
         println!("The message is herei: {} {} ", self.x, self.y);
     }
}

fn main () {

  MoveMessage::call(&MoveMessage { x: 1, y: 2 });
}
