#[derive(Debug)]
struct DeliSandwich {}

fn main() {
    println!("{}", identity(5));
    println!("{}", identity(5.4563));
    println!("{}", identity("Hello"));
    println!("{}", identity(true));
    println!("{}", identity(String::from("Hello")));
    println!("{:?}", identity(DeliSandwich {}));
}

fn identity<T>(value: T) -> T {
    value
}

// fn identity_i32(value: i32) -> i32 {
//     value
// }
// fn identity_bool(value: bool) -> bool {
//     value
// }
