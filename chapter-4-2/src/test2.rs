fn main() {
    let s1 = String::from("Rust is rolling");
    let s2 = s1.clone();
    println!("S1 is , {}", s1);
    println!("{s2}");


    // test references:
    //
    let sv = String::from("Rust volumes");
    let s_len = calculate_length(&sv);

}

fn calculate_length(s: &String)-> usize {
   
     s.len();
  
}
