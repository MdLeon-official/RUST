use std::collections::HashSet;
use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");
    let unique_chars: HashSet<char> = inp.trim().chars().collect();
    if unique_chars.len() % 2 == 0 {
        println!("CHAT WITH HER!")
    } else {
        println!("IGNORE HIM!")
    }
}
