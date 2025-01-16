use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");
    let numbers: Vec<i64> = inp
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to get user input."))
        .collect();
    let a = numbers[0];
    let b = numbers[1];
    let c = numbers[2];
    let d = numbers[3];

    let a_to_b = (a as f64).ln() * (b as f64);
    let c_to_d = (c as f64).ln() * (d as f64);

    if a_to_b > c_to_d {
        println!("YES");
    } else {
        println!("NO");
    }
}
