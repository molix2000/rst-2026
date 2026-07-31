 struct QuitMessages;
 struct MoveMessages {

     x: i32,
     y: i32,
     z: i32,
 }


struct WriteMessage;

impl WriteMessage {
    fn messagewrite(&self) -> String {
        let s = "This is the messagewrite function from within WriteMessage".to_string();
        println!("{}", s);
        s
    }
}

impl QuitMessages {

    fn call_quit (&self) {
          let m = WriteMessage.messagewrite();
    }
}

fn main() {


   let msgw = WriteMessage ;
   let msg1 = "This is life".to_string();
   println!("This is the output of WriteMessage {}", msgw.messagewrite());
   let qmsq = QuitMessages;
    qmsq.call_quit();
}


