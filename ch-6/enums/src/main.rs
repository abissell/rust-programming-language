#![allow(dead_code)]

#[derive(Debug)]
enum UsState {
    California,
    Montana,
    Washington,
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
        Coin::Quarter(_) => 25,
    }
}

fn main() {
    println!("Hello, world!");
    println!("Value of quarter: {}", value_in_cents(Coin::Quarter(UsState::California)));

    let some_u8_value = 3u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let other_u8_value = Some(3u8);
    if let Some(3) = other_u8_value {
        println!("three");
    }
}
