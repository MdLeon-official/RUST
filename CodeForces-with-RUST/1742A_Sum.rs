use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");

    let loops: i32 = inp.trim().parse().expect("failed to read input.");
    for i in 0..loops {
        let mut time = String::new();
        io::stdin()
            .read_line(&mut time)
            .expect("Failed to read input");
        let time_vec: Vec<i64> = time
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to read input"))
            .collect();
        let max = time_vec.iter().max().unwrap();

        let sum: i64 = time_vec.iter().sum();
        if *max == sum-*max {
            println!("YES");
        }else {
            println!("NO");
        }
    }
}
