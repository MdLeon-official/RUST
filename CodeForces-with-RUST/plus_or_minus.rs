use std::io;

fn main() {
    let mut inp_num = String::new();

    io::stdin()
        .read_line(&mut inp_num)
        .expect("Failed to read input");
    let t: i32 = inp_num.trim().parse().expect("Failed to parse input");

    for _ in 0..t {
        let mut no_use = String::new();
        io::stdin()
            .read_line(&mut no_use)
            .expect("Failed to read input");

        let nums: Vec<i64> = no_use
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to read input"))
            .collect();
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];

        if a + b == c {
            println!("+")
        }
        else if a - b == c || b - a == c{
            println!("-")
        }
        
    }
}
