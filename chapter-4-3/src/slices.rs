pub fn main() {

   let slize = String::from("Rust Slices");
   fstwrd(&slize);
   

}

pub fn fstwrd(s: &String)-> usize {
    let by = s.as_bytes();
    for (i,item) in bytes.iter().enumerate() {
       if item == b' ' {
           return i;
       }
    }
  s.len()
}
