enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }
    println!("{count}");

    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

    println!("{count}");
}
