use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let num = input
        .trim()
        .parse()
        .expect("Please provide a valid integer");
    for i in 1..=num {
        println!("{}", i);
    }
}
