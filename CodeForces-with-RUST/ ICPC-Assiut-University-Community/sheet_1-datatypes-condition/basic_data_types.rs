use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get user input");

    let values: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();

    for value in values {
        println!("{}", value);
    }
}
