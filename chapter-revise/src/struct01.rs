#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,

}

impl Rectangle {
    fn run(&self, x: u32, y: u32) -> u32 {
        x + y
    }
}

fn main(){

   let rect1 = Rectangle{
       width : 44,
       height : 31,
   };

  println!("The rect1 dimentions are {:#?}", &rect1);
  println!("The rect1 run results are {:#?}", rect1.run(12,33));

}
