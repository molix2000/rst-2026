fn stringequal() {
    let s1: String = String::from("Rust Less Risk");
    let  s2 = s1;

    println!("S2 is {s2}");

    let s3 = String::from("Rust Road");
    let s4 = s3.clone();
    println!("s3 = {s3}, s4 = {s4}");

    let x = 5;
    let y = x;

    println!("X = {x}, Y = {y}");


    let s5 = String::from("Rust topics");
    takes_ownership(s3);
    let x = 5;
    makes_copy(x);
    takes_and_gives_back(s4);
    // calculate_length(s5);
}

pub fn takes_ownership(some_string: String){
        println!("{some_string}");

}

pub fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
pub fn takes_and_gives_back(a_string: String) -> String{
       a_string
}
//pub fn calculate_length(s: String) -> (String, usize){
  //   let length = s.len();
  //   (s, length)
//}
