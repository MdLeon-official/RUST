use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");

    let numbers: Vec<i64> = inp
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to read input."))
        .collect();

    let a: i128 = numbers[0] as i128;
    let b: i128 = numbers[1] as i128;
    let c: i128 = numbers[2] as i128;
    let d: i128 = numbers[3] as i128;

    let multi: i128 = a * b * c * d;
    println!("{:02}", multi % 100);
}
