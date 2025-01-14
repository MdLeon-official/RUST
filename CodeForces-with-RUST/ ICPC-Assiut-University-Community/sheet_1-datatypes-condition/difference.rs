use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let my_int: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to read input"))
        .collect();

    let num0 = my_int[0];
    let num1 = my_int[1];
    let num2 = my_int[2];
    let num3 = my_int[3];

    println!("Difference = {}", (num0 * num1) - (num2 * num3));
}
