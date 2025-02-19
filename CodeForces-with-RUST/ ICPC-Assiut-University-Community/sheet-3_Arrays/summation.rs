use std::io;

fn main() {
    let mut first_inp = String::new();
    io::stdin().read_line(&mut first_inp).expect("Failed to read input");

    let mut second_inp = String::new();
    io::stdin().read_line(&mut second_inp).expect("Failed to read input");

    let sec_vec: Vec<i64> = second_inp
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to read input"))
        .collect();
    let mut sum= 0;
    for i in sec_vec {
        sum = sum + i;
    }
    println!("{}", sum.abs());
}
