use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let numbers: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Parse error"))
        .collect();

    println!("{} {}", numbers.iter().min().unwrap(), numbers.iter().max().unwrap())
}
