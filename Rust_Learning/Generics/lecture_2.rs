#[derive(Debug)]
struct DeliSandwich {}

fn main() {
    println!("{}", identity::<i64>(5));
    println!("{}", identity::<i8>(5));
    println!("{}", identity::<u32>(5));
    println!("{}", identity::<f64>(5.4563));
    println!("{}", identity::<&str>("Hello"));
    println!("{}", identity::<bool>(true));
    println!("{}", identity::<String>(String::from("Hello")));
    println!("{:?}", identity::<DeliSandwich>(DeliSandwich {}));
}

fn identity<T>(value: T) -> T {
    value
}
