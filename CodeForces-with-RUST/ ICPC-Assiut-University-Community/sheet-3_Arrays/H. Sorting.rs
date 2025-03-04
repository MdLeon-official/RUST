use std::io;

fn main() {
    let mut iter = String::new();
    io::stdin()
        .read_line(&mut iter)
        .expect("Failed to read line");
    let mut user_sp = String::new();
    io::stdin()
        .read_line(&mut user_sp)
        .expect("Failed to read line");
    let mut number: Vec<i64> = user_sp
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    number.sort();
    for num in &number {
        print!("{} ", num);
    }
    print!("\n");
}
