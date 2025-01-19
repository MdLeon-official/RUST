fn main() {
    let present_value = Option::Some(13);
    let missing_value: Option<i32> = None;

    println!("{}", present_value.unwrap_or(0));
    println!("{}", missing_value.unwrap_or(0));
}
