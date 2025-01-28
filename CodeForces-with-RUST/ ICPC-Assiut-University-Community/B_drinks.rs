use std::io;

fn main() {
    let mut sum = 0;
    let mut user = String::new();
    io::stdin()
        .read_line(&mut user)
        .expect("Failed to read input.");
    let n: i32 = user.trim().parse().expect("Failed to read input");

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");
    let number: Vec<i64> = inp
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Please provide a valid input"))
        .collect();
    for i in number {
        sum = sum + i
    }
    let answer: f64 = sum as f64 / n as f64;
    println!("{answer:12}")

}
