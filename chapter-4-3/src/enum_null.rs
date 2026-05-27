enum Option<T> {
     None,
     Some(T),
}

fn main() {

   let x : i8 =5;
   let y: Option<i8> = Some(5);
   let sum = x + y;
   println!("The SUM is : {}", sum);
   let some_number = Some(5);
   let some_char = Some('v');
   let absent_number = Options<i32> = None;


}
