use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");

    let loops = inp.trim().parse().expect("failed to read input.");
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
        let mut h = time_vec[0];
        let mut m: i64 = time_vec[1];
        if 0 <= h && h < 24 {
            h = (24 - h) * 60;
        }
        if 0 <= m && m < 60 {
            m = 60 - m;
        }
        println!("{}", (h + m) - 60);
    }
}
