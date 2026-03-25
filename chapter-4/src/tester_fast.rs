fn main() {
    let owned_string = get_string();
    print_string(&owned_string);
}

fn get_string() -> String {
    let s = String::from("Hello, World!");
    // In Rust we return s by omitting the semicolon
    s
}

fn print_string(my_string: &str) {
    println!("{}", my_string);
}

