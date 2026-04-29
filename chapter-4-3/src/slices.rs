pub fn main() {

   let slize = String::from("Rust Slices");
   fstwrd(&slize);
   let zlice = String::from("Rust Zlice");
   let fist = &zlice[0..4];
   let last = &zlice[5..10];
   let tester = &zlice[..];
   println!(" The 1st of zlice is {fist}, the last of Zlice is {last}, tester is {tester}");

}

pub fn fstwrd(s: &String)-> (&str) {
    let by = s.as_bytes();
    let slize2 = &s[1..3];
    assert_eq!(slice,& [2,3]);
    for (i,&item) in by.iter().enumerate() {
       if item == b' ' {
           return &s[0..i];
       }
    }
  s.len()
}
