use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");
    let mut numbers: Vec<i64> = inp
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to get user input."))
        .collect();
    let mut sorted = numbers.clone();
    sorted.sort();
    for i in 0..3 {
        println!("{:?}", sorted[i]);
    }
    println!("");
    for i in 0..3 {
        println!("{:?}", numbers[i]);
    }
}
