#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
     fn boleanise (&self) -> bool {
        self.width > 0
     }

}

fn main(){
   let rect1 = Rectangle {

       height: 33,
       width: 43,
   };

   println!("This is the rect1 {:#?}", rect1);
   println!("This is the boleanise result output {:#?}", rect1.boleanise());
}
