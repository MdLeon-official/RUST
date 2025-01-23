use std::io;

fn main() {
    let mut inp = String::new();
    let mut even = 0;
    let mut odd = 0;
    let mut positive = 0;
    let mut negative = 0;

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");

    let loops: i32 = inp.trim().parse().expect("failed to read input.");
    let mut time = String::new();
    io::stdin()
        .read_line(&mut time)
        .expect("Failed to read input");
    let time_vec: Vec<i64> = time
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to read input"))
        .collect();
    for j in time_vec {
        if j > 0 {
            positive += 1;
        } else if j < 0 {
            negative += 1;
        }

        if j % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }
    println!("Even: {}", even);
    println!("Odd: {}", odd);
    println!("Positive: {}", positive);
    println!("Negative: {}", negative);
}
