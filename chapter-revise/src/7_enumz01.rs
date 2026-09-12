#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The Sate Quarter from {:#?}", state);
            25
        }
    }
}

fn main() {
    println!(
        "The function all is here for Penny {:#?}",
        value_in_cents(Coin::Penny)
    );
    println!(
        "The function all is here for Nickel {:#?}",
        value_in_cents(Coin::Nickel)
    );
    println!(
        "The function all is here Dime {:#?}",
        value_in_cents(Coin::Dime)
    );
    println!(
        "This function all is here Quarter {:#?}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
}
