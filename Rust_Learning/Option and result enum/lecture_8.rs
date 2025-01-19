#[derive(Debug, Clone, Copy)]
enum MyOption {
    Some(i32),
    None,
}
impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh oh"),
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}
fn main() {
    let some_option = MyOption::Some(100);
    println!("{}", some_option.unwrap());
    println!("{:?}", some_option);
    println!("{}", some_option.unwrap_or(10));

    let none_option = MyOption::None;
    // println!("{}", none_option.unwrap());
    println!("{}", none_option.unwrap_or(3245214));
}
