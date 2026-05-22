enum msg {

     Quit,
     Move {x:i32, y:i32},
     Write (String),
     ChangeColor(i32, i32,i32),

}


fn call(&self) {
     // Quit;
     // Move();
     // Write();


}

impl msg {

   fn call(&self) {

   }
}

fn main() {
   
     let m = msg::Write(String::from("Hello"));

}
