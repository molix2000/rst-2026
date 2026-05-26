#[derive(Debug)]
enum Msg {

     Quit,
     Move {x:i32, y:i32},
     Write((String)),
     ChangeColor(i32, i32,i32),

}


//fn call(&self) {
     // Quit;
     // Move();
     // Write();


//}

impl Msg {

   fn call(&self) {

   }
}

fn main() {
   
     let m = Msg::Write(String::from("Hello"));
     println!("The value of m is {:#?}", m);
     let n = (Msg::Move {x:10, y:50}.call());
     println!("The value of n is {:#?}", n);
}
