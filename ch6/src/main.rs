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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn some_u8_value_one(x: Option<u8>) {
    match x {
        Some(3) => println!("Three!"),
        _ => ()
    }
}

fn some_u8_value_two(x: Option<u8>) {
    if let Some(3) = x {
        println!("Three!");
    }
}

fn count_non_quarter_coins(coin: Coin) -> i32 {
    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    count
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:#?}", home);
    println!("{:#?}", loopback);

    println!("Coin value for {:?}: {}", Coin::Penny, value_in_cents(Coin::Penny));
    println!("Coin value for {:?}: {}", Coin::Nickel, value_in_cents(Coin::Nickel));
    println!("Coin value for {:?}: {}", Coin::Dime, value_in_cents(Coin::Dime));
    println!("Coin value for {:?}: {}\n", Coin::Quarter(UsState::Alaska), value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}, {:?}, {:?}", five, six, none);

    some_u8_value_one(Some(0u8));
    some_u8_value_two(Some(0u8));

    let mut count = count_non_quarter_coins(Coin::Quarter(UsState::Alaska));
    println!("Count: {:?}", count);
    count = count_non_quarter_coins(Coin::Quarter(UsState::Alaska));
    println!("Count: {:?}", count);
    count = count_non_quarter_coins(Coin::Dime);
    println!("Count: {:?}", count);
}
