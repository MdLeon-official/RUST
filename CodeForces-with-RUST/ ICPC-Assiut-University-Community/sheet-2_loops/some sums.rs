use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (n, a, b) = (nums[0], nums[1], nums[2]);

    let mut sum = 0;
    for i in 1..=n {
        let digit_sum = get_digit_sum(i);
        if digit_sum >= a && digit_sum <= b {
            sum += i;
        }
    }

    println!("{}", sum);
}

fn get_digit_sum(mut num: i32) -> i32 {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}
