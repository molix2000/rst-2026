#[derive(Debug)]
struct Rectangle {
   width: u32,
   height: u32,

}

fn main() {
   let scale = 2;
   let rect1 = Rectangle {
       width: dbg!(30 * scale),
       height: 44,
   };     
   
   println!("The dbg resutl for rect1 is {:#?}",dbg!(rect1));

}
