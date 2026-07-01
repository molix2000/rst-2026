#[derive(Debug)]
struct Rect {

    width: u32,
    height: u32,
}

fn main () {
    let rect1 = Rect {

        width: 30,
        height: 45,

    };

 let scale = 2;
 let rect2 = Rect {
     width : dbg!(30 * scale),
     height: 50,
 };
    
    println!("The area of the rectangle {}", area(&rect1));
    println!("The areat result is  {:?}", areat((rect1.width, rect1.height)));
    println!("The rect2 management is {:?}", areat((rect2.width, rect2.height)));
}

fn area (rectangle: &Rect) -> u32 {

   rectangle.width * rectangle.height
}

fn areat (tup: (u32, u32)) -> u32 {

   tup.0 * tup.1
}

//fn areas<'a> (tup: (&'a u32,&'a u32)) -> &'a u32 {
//
//    &(tup.0 * tup.1)
//}
