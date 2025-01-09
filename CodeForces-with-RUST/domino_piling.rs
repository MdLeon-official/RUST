use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let number: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Please provide a valid input"))
        .collect();
    let _num1 = number[0];
    let _num2 = number[1];

    let square = _num1 * _num2;
    let ans = (square as f64 / 2.0).floor();
    println!("{}", ans);
}
