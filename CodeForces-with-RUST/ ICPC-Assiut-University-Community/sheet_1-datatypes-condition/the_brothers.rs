use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let name_1: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let ln1 = name_1[1].clone();

    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input.");
    let name_2: Vec<String> = input1
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let ln2 = name_2[1].clone();

    if ln1 == ln2 {
        println!("ARE Brothers")
    } else {
        println!("NOT")
    }
}
