fn main() {

  let s = String::from("Rust Changes String");
  change(s);

}

fn change(mut some_string: String) {

   some_string.push_str(" , Time");
   println!("{}", some_string);

}
