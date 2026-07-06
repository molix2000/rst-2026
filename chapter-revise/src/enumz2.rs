#[derive(Debug)]
enum IpAddr {
    V4(()),
    V6(()),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

fn main() {
    let home = IpAddr::V4(());
    let loop_back = IpAddr::V6(());

    println!("The home one is {:#?}", home);
    println!("The LoopBack is {:#?}", loop_back);
}