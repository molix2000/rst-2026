fn main() {
    let s = String::from("hello");
    let leng = calculate_length(&s);
    println!("The length is {}",leng);
}

//fn change(some_string:  &String) {
//some_string.push_str(", world");
//}

fn calculate_length(s: &String) -> usize {
    s.len()
}
