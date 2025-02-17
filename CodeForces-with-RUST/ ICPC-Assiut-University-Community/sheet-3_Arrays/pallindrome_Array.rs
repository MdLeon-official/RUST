use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: i32 = input.trim().parse().expect("Failed to read input");

    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    let numbers: Vec<i64> = input2
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to read input"))
        .collect();

    let length = numbers.len() / 2;
    let mut is_palindrome = true;

    for i in 0..length {
        if numbers[i] != numbers[numbers.len() - 1 - i] {
            is_palindrome = false;
            break;
        }
    }

    if is_palindrome {
        println!("YES");
    } else {
        println!("NO");
    }
}
