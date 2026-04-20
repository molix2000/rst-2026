pub fn main() {
   let str1 = String::from("Rust Borrow");
   let len = calculate_len(&str1);

   println!("The length of str1 s {len}");

}

pub fn calculate_len(s: &String) -> usize {

   s.len()
} 
