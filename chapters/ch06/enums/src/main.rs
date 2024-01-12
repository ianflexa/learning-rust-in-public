#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

impl IpAddrStruct {
    fn new(kind: IpAddrKind, address: String) -> Self {
        Self { kind, address }
    }
}

#[derive(Debug)]
// representing the same concept using just an enum is more concise
enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // no data associated
    Move { x: i32, y: i32 },    // has named fields
    Write(String),              // includes a single String
    ChangeColor(i32, i32, i32), // includes three i32 values
}

impl Message {
    fn call(&self) {}
}

// Rust does not have nulls, but it does have an enum that can encode 
// the concept of a value being present or absent
enum Option0<T> { // is defined by the std lib
    None,
    Some(T),
}
fn main() {
    // enum values;
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four! {:#?}", four);
    println!("six! {:#?}", six);

    let home = IpAddrStruct::new(IpAddrKind::V4, String::from("127.0.0.1"));
    println!("home {:#?}", home);

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("loopback {:#?}", loopback);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option

    let some_num = Some(5);
    let some_char = Some('c');
    let absent_number: Option<i32> = None;



    let x: i8 = 2;
    let y: Option<i8> = Some(1);
    // let sum = x + y;
                  // ^ no implementation for `i8 + Option<i8>`

}

fn route(ip_kind: IpAddrKind) {}

// **** notes *****
/*
- enums give you a way of saying a value is one of a possible set of values
- the name of each enum variant that we define also becomes a function that constructs an instance of the enum.
- we can put any kind of data inside an enum variant
- Rust doesn’t have the `null` feature that many other languages have.
- The <T> syntax is a generic type parameter
- Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null. 
*/
