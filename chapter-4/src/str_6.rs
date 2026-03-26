fn main() {
    let mut str_slice: &str = "Rust Book";
    let owned_str: String = str_slice.to_string();
    let owned_str_alternative = String::from(str_slice);

    println!("{}", owned_str);
    println!("{}", owned_str_alternative);
}
