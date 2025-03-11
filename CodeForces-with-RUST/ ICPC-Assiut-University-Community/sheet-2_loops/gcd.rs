use std::io;

fn main() {
    let mut user_sp = String::new();
    io::stdin()
        .read_line(&mut user_sp)
        .expect("Failed to read line");
    let number: Vec<i64> = user_sp
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    let a = number[0];
    let b = number[1];

    gcd(a, b);
}

fn gcd(a: i64, b: i64) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for i in 1..=a {
        if a % i == 0 {
            vec1.push(i);
        }
    }
    for j in 1..=b {
        if b % j == 0 {
            vec2.push(j);
        }
    }
    let result: Vec<i64> = vec1
        .iter()
        .filter(|&&x| vec2.contains(&x) && vec1.iter().filter(|&&y| y == x).count() == 1)
        .cloned()
        .collect();
    let max = result.iter().max().unwrap();
    println!("{}", max);
}
