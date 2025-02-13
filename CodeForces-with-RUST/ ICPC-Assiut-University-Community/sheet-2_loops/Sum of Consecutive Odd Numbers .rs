use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");
    let iterate = inp.trim().parse().expect("Failed");

    for _ in 0..iterate {
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

        let mut sum = 0;
        let (start, end) = if a < b { (a, b) } else { (b, a) };

        for i in (start + 1)..end {
            if i % 2 != 0 {
                sum += i;
            }
        }
        println!("{}", sum);
    }
}
