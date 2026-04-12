fn main() {
   let s1: String = String::from("Rust String 01");
   let len = calc_len(&s1);
   println!("The len is {len} the String is {&s1}"); 
}

fn calc_len(s: &String) -> (s.ToString(), usize){
   let len = s.len();
   (s, len)
}
