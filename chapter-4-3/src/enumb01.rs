#[derive(Debug)]

enum Addrvers {
     V4,
     V6,

}

#[derive(Debug)]
struct IpAdd {
  kind: Addrvers,
  address: String,

}



fn main() {

    let home = IpAdd {
        kind: Addrvers::V4,
        address: String::from("127.0.0.1"),


     };


    let looper = IpAdd {
        kind: Addrvers::V6,
        address: String::from("::1"),

     };
    println!("The Home components are {:#?}", home);
    println!("The Looper components are {:#?}", looper);
    println!("Looper version is {}"looper::kind);
}








