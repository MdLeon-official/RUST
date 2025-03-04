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
    let number: Vec<i64> = user_sp
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    let minimum: i64 = *number.iter().min().unwrap();
    let mut position = number.iter().position(|&r| r == minimum).unwrap();
    
    println!("{} {}", minimum, position+1);
}
