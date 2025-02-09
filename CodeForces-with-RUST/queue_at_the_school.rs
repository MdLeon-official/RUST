use std::io;

fn main() {
    let mut user_inp = String::new();
    io::stdin()
        .read_line(&mut user_inp)
        .expect("Failed to read input");
    let nums: Vec<i64> = user_inp
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to read"))
        .collect();
    let _n = nums[0];
    let t = nums[1];

    let mut str_inp = String::new();
    io::stdin()
        .read_line(&mut str_inp)
        .expect("Failed to read input");
    let mut queue: Vec<char> = str_inp.trim().chars().collect();

    for _ in 0..t {
        let mut i = 0;
        while i < queue.len() - 1 {
            if queue[i] == 'B' && queue[i + 1] == 'G' {
                queue.swap(i, i + 1);
                i += 2;
            } else {
                i += 1;
            }
        }
    }

    println!("{}", queue.iter().collect::<String>());
}
