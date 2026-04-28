pub fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
            println!("The first word is {}", &s[0..i]);
        }
    }
    s.len()
}
