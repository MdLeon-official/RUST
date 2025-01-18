use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");
    let numbers: Vec<&str> = inp
        .trim()
        .split(".")
        .collect();
    if numbers.len() == 2 {
        let integer = numbers[0];
        let decimal = numbers[1];

        if decimal.chars().all(|a| a == '0') {
            println!("int {}", integer);
        }else {
            println!("float {} 0.{}", integer, decimal);
        }
    }
    else {
        println!("int {}", numbers[0]);
    }
}
