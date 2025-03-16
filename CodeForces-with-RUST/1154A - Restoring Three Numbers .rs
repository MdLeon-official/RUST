use std::io;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    nums.sort();

    let c = nums[3] - nums[0];
    let b = nums[3] - nums[1];
    let a = nums[3] - nums[2];

    println!("{} {} {}", a, b, c);
}
