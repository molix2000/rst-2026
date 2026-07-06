#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
    V8,
}

struct Ipaddr {
    kind: IpAddrKind,
    address: String,
}

pub fn enumz() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let eight = IpAddrKind::V8;

    println!("This is the four{:#?}", IpAddrKind::V4);
    println!("This is the six {:#?}", IpAddrKind::V6);
    println!("This is the 8th {:#?}", IpAddrKind::V8);

    println!("The route applied for 4th {:#?}", ipaddr_route(four));

    let home_ip_addr = Ipaddr {
        kind: IpAddrKind::V4,
        address: "10.10.0.1".to_string(),
    };

    let loopback = Ipaddr {
        kind: IpAddrKind::V6,
        address: "127.0.0.1".to_string(),
    };
}

pub fn ipaddr_route(ip_kind: IpAddrKind) -> IpAddrKind {
    ip_kind
}
