 #[derive(Debug)]

// There is one place to put derive debug, its up there.
enum IpAddrKind {
     V4,
     V6,

}

#[derive(Debug)]

enum Message {

 Quit,
 Move{X: i32, y: i32},
 Write(String),
 ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { X, y } => println!("Move to X: {}, y: {}", X, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("ChangeColor: {},{},{}", r, g, b),
        }
    }
}

#[derive(Debug)]
struct IpAddr {
   kind: IpAddrKind,
   address: String,

}
fn main() {


  let home = IpAddr {
      kind: IpAddrKind::V4,
      address: String::from("127.0.0.1"),
  };

  let localoop = IpAddr {
      kind: IpAddrKind::V6,
      address: String::from("::1"),
  
  };

    //#[derive(Debug)]

    //fn main (){
    //#[derive(Debug)]
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let m = Message::Write(String::from("Raspberrypi 5"));
     m.call();
    println!("V4 is {:#?}",four);
    println!("V6 is {:#?}",six);
    println!("home address is {:#?}", home.address);
    println!("The entire home struct is {:#?}", home);
    println!("The local loop kind is {:#?}", localoop.kind);
    println!("m is {:?}", m);
}


