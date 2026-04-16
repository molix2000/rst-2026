pub fn mutabex() {
    let mut s = String::from("hello");
    let leng = calculate_length(&s);
    println!("The length is {}",leng);
    let mut s1 = String::from("Rust String1");
    println!("S1 is {s1}"); 
    {
          let r1 = &mut s1;
    }
          let r2 = &mut s1;
          println!(" r2 is {r2}");

          let r3 = &mut s;
          println!("print r3 {r3}");
}

//fn change(some_string:  &String) {
//some_string.push_str(", world");
//}


pub fn calculate_length(s: &String) -> usize {
    s.len()
}
