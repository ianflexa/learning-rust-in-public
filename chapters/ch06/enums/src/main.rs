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
        address: String::from("::1")
    };
    println!("loopback {:#?}", loopback);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));


}

fn route(ip_kind: IpAddrKind) {}

// **** notes *****
/*
    - enums give you a way of saying a value is one of a possible set of values
    - the name of each enum variant that we define also becomes a function that constructs an instance of the enum.
*/
