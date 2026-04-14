pub fn changes() {

  let s = String::from("Rust Changes String");
  change(s);

}

pub fn change(mut some_string: String) {

   some_string.push_str(" , Time");
   println!("{}", some_string);

}
