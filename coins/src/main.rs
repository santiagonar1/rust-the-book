#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State {:?}", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Nickel;
    value_in_cents(coin);
    let coin = Coin::Dime;
    value_in_cents(coin);
    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin);

    // Equivalent expressions:
    let coin = Coin::Quarter(UsState::Alabama);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => println!("Not a quarter"),
    }

    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        println!("Not a quarter");
    }
}
