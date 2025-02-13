use std::io;

fn main() {
    let mut condition = true;

    while condition {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        let nums: Vec<i64> = user_input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to read input"))
            .collect();

        let a = nums[0];
        let b = nums[1];

        if a <= 0 || b <= 0 {
            condition = false;
            continue;
        }

        let mut sum = 0;
        let (start, end) = if a < b { (a, b) } else { (b, a) };

        for i in start..=end {
            print!("{} ", i);
            sum += i;
        }
        println!("sum ={}", sum);
    }
}
