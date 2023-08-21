#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

#[derive(Debug)]
enum IpAddrInfo {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {}
struct Ipv6Addr {}

enum IpAdd4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        println!("hhhh");
    }
}

pub fn enums01() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four:{:?},six:{:?}", four, six);
    route(four);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        addr: String::from("::1"),
    };
    println!("home:{:?},loopback:{:?}", home, loopback);
    let home = IpAddrInfo::V4(String::from("127.0.0.1"));
    let loopback = IpAddrInfo::V6(String::from("::1"));
    println!("home:{:?},loopback:{:?}", home, loopback);

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));
    println!("home:{:?},loopback:{:?}", home, loopback);

    let v4 = Ipv4Addr {};
    let v6 = Ipv6Addr {};
    let home = IpAdd4::V4(v4);
    let loopback = IpAdd4::V6(Ipv6Addr {});
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents02(Coin::Penny));
    println!("{}", value_in_cents02(Coin::Quarter));
    println!("{}", value_in_cents03(Coin02::Quarter(UsState::Alabama)));

    println!("{:?}", plus_one(Some(5)));
    println!("{:?}", plus_one(None));
}

fn route(ip_type: IpAddrKind) {
    println!("ip_type:{:?}", ip_type);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
// 模式匹配
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn value_in_cents02(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin02 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents03(coin: Coin02) -> u8 {
    match coin {
        Coin02::Penny => 1,
        Coin02::Nickel => 5,
        Coin02::Dime => 10,
        Coin02::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
