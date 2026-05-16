#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}

struct IpAddr {

    kind: IpAddrKind,
    address: String,
}


fn main() {


    let home = IpAddr {
    kind: IpAddrKind::v4,
    address: String::from("127.0.0.1"),

    };


    let loopback = IpAddr {
    kind: IpAddrKind::v6,
    address: String::from("::1"),

    };



    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;
    println!("This is four {:?} ", IpAddrKind::v4);
    println!("The sixth value is {:#?}", IpAddrKind::v6);

}
