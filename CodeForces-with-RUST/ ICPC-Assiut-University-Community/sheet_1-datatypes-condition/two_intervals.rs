use std::io;
use std::cmp;

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

    let a: i64 = numbers[0];
    let b: i64 = numbers[1];
    let c: i64 = numbers[2];
    let d: i64 = numbers[3];

    let start = cmp::max(a,c);
    let end = cmp::min(b, d);

    if start <= end {
        println!("{start} {end}");
    }else {
        println!("-1");
    }
    
}
