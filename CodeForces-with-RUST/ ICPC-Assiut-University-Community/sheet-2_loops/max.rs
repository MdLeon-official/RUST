use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read message.");
    let mut again_inp = String::new();
    io::stdin()
        .read_line(&mut again_inp)
        .expect("Failed to read input.");
    let numbers: Vec<i64> = again_inp
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to read input"))
        .collect();
    println!("{}", numbers.iter().max().unwrap());
}
