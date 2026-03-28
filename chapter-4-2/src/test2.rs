fn main() {
    let s1 = String::from("Rust is rolling");
    let mut s2 = s1.clone();
    println!("S1 is , {}", s1);
    println!("{s2}");

    // test references:
    //
    let mut sv = String::from("Rust volumes");
    let sv_len = calculate_length(sv.clone());
    return_val( sv);
    change( &mut s2);
    
}

fn calculate_length(sv: String)-> usize {
    sv.len()
}

fn return_val(s: String) -> String {
   println!("{s}");
   s
}


fn change(some_string: &mut String) {
    some_string.push_str(" .. Rust");


}
