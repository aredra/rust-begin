#[derive(Debug)]
enum KrState {
    Seoul,
    Busan,
    Daegu,
    Incheon,
    Gwangju,
    Daejeon,
    Ulsan,
    Sejong
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(KrState),
}

fn main() {
    let mut coin = value_in_cents(Coin::Penny);
    println!("{}", coin);
    coin = value_in_cents(Coin::Quarter(KrState::Seoul));
    println!("{}", coin);
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        _ => None
    }
}