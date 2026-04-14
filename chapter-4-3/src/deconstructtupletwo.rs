 fn main() {
    let s1 = String::from("Rust deconstruct");
    let len = calculate_length(&s1); 
    
    println!("The length of '{s1}' is {len}.");
}
fn calculate_length(s: &String) ->  usize {
    s.len()
    
}
