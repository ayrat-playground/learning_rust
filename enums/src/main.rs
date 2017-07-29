#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddr1 {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Washington
}

fn main() {
    let home = IpAddr{kind: IpAddrKind::V4, address: String::from("127.0.0.1")};
    println!("Home ip is {:?}", home);

    let home1 = IpAddr1::V4(127, 0, 0, 1);
    println!("Home ip is {:?}", home1);

    println!("Dime is {} in cents", value_in_cents(Coin::Dime));

    let quarter = Coin::Quarter(UsState::Alaska);
    value_in_cents(quarter);

    let five = Some(5);
    let six = plus_one(five);
    println!("Number is {:?}", six);

    if let Coin::Quarter(state) = Coin::Quarter(UsState::Alabama) {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Coin without state");
    }
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None    => None,
        Some(i) => Some(i + 1)
    }
}
