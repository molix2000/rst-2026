pub fn dangle(){
     let zind = String::from("Rust Test 01");
     let reference_to_nothing = dangles(zind);
     println!("{reference_to_nothing}");

}

pub fn dangles(_s: String) -> String {
   let s = String::from("Rust domain new");
   s
}

// Errot riddled structure before:
// fn dangle() -> String {
// fn dangle(&String) &String { // other variant.
//  let s = String::from("Rust");
//  &s
// }
