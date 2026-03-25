fn main() {
   let owned_string = get_string();
   print_string(&owned_string);

}

fn get_string() -> String {

  let s = String::from("Rust Strings");
  // Rust returns s by omitting the ;
  s
}


fn print_string(my_string: &str) {
   println!("{}",&my_string);

   println!("{}",my_string.to_uppercase());
   println!("{}",&my_string.to_uppercase());
}

