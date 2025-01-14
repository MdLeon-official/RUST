use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let expression = input.trim();

    if expression.contains('+') {
        let nums: Vec<i64> = expression.split('+')
            .map(|x| x.parse().expect("Failed to parse number"))
            .collect();
        println!("{}", nums[0] + nums[1]);
    } else if expression.contains('-') {
        let nums: Vec<i64> = expression.split('-')
            .map(|x| x.parse().expect("Failed to parse number"))
            .collect();
        println!("{}", nums[0] - nums[1]);
    } else if expression.contains('*') {
        let nums: Vec<i64> = expression.split('*')
            .map(|x| x.parse().expect("Failed to parse number"))
            .collect();
        println!("{}", nums[0] * nums[1]);
    } else if expression.contains('/') {
        let nums: Vec<i64> = expression.split('/')
            .map(|x| x.parse().expect("Failed to parse number"))
            .collect();
        println!("{}", nums[0] / nums[1]);
    }
}
