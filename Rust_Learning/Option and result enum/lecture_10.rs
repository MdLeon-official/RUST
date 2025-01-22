fn main() {
    let text = "50";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);

    let text = "Alabama";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);
}
