use regex::Regex;

fn main() {

   let pattern =  Regex::new(r"\d+").unwrap();
   let text = "I have 3 apples, 5 oranges, and 12 bananas";
   for match_ in pattern.find_iter(text) {
       println!("{}", match_.as_str());
   }
 let sir = "High Rust";
 for c in sir.chars() {
    println!("{}", c);
 }   

let sim = "Rust Clinic";
    for b in sim.bytes() {

     println!("{}", b);
    }

let mut s = String::from("Rust Books");
let mut new_s  = s.push_str(" cycle");
println!("{new_s}");



}

