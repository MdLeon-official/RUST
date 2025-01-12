use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let numbers: Vec<f64> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Parse error"))
        .collect();

    let a = numbers[0];
    let b = numbers[1];

    let floor = (a / b).floor() as i64;
    let ceil = (a / b).ceil() as i64;
    let round = (a / b).round() as i64;

    println!("floor {} / {} = {}", a as i64, b as i64, floor);
    println!("ceil {} / {} = {}", a as i64, b as i64, ceil);
    println!("round {} / {} = {}", a as i64, b as i64, round);
}
