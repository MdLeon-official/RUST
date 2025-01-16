fn main() {
    let a = Option::Some(5);
    let b = Option::Some("Hello");
    let c = Option::Some(true);

    let a = Option::<i16>::Some(5);

    let d: Option<&str> = Option::None;
}
