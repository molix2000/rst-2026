fn main() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = 'D';

    println!("The c char is {c}");
    println!("The z char is {z}");
    println!("The heart eye is {heart_eyed_cat}");
    let mut a: (i32,f64,u8) = (500,6.4,8);
    let (x,y,z) = a;
    let five_oo = a.0;
    let six_4 = a.1;
    let eight = a.2;
    main2();
    another_fn(8);
    delta();
}

fn main2() {
  // create some strings
  let string1 = " Welcome to Edpresso    ";
  let string2 = "Educative is the best!     ";
  let string3 = "     Rust is very interesting!";

  // trim the strings
  let trim1 = string1.trim();
  let trim2 = string2.trim();
  let trim3 = string3.trim();

  // print the trims
  println!("The string before trim is '{}' and length is {}", string1, string1.len());
  println!("The string when trimmed is '{}' and length is {}", trim1, trim1.len());

  println!("\nThe string before trim is '{}' and length is {}", string2, string2.len());
  println!("The string when trimmed is '{}' and length is {}", trim2, trim2.len());

  println!("\nThe string before trim is '{}' and length is {}", string3, string3.len());
  println!("The string when trimmed is '{}' and length is {}", trim3, trim3.len());

}

fn another_fn(x: i32) -> i32 {
     let v = x + 5;
     println!("The value of V is {v}");
     return v ;
     
     
}


fn delta() {

    let a = [0,1,2,3,4,5];
    let mut index = 0;

    while index < 6 {
      println!("the position ofd index is{index}");
      index += 1;    
    }
}

fn printer_two(){

    let 
}

// this url https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html