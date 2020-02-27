#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:#?}", home);
    println!("{:#?}", loopback);

    println!("Coin value for {:?}: {}", Coin::Penny, value_in_cents(Coin::Penny));
    println!("Coin value for {:?}: {}", Coin::Nickel, value_in_cents(Coin::Nickel));
    println!("Coin value for {:?}: {}", Coin::Dime, value_in_cents(Coin::Dime));
    println!("Coin value for {:?}: {}", Coin::Quarter(UsState::Alaska), value_in_cents(Coin::Quarter(UsState::Alaska)));
}
