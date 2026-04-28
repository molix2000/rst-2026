pub fn main() {

   let slize = String::from("Rust Slices");
   fstwrd(&slize);
   let zlice = String::from("Rust Zlice");
   let fist = &zlice[0..4];
   let last = &zlice[5..10];
   println!(" The 1st of zlice is {fist}, the last of Zlice is {last}");

}

pub fn fstwrd(s: &String)-> usize {
    let by = s.as_bytes();
    for (i,&item) in by.iter().enumerate() {
       if item == b' ' {
           return i;
       }
    }
  s.len()
}
