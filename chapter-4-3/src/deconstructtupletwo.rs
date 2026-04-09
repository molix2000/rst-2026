fn main() {
    let s1 = String::from("Rust deconstruct");
    // We pass a reference (&s1) instead of the whole string
    let len = calculate_length(&s1); 
    
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
