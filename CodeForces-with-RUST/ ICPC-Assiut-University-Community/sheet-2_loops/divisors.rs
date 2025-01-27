use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");

    let n: i32 = inp.trim().parse().expect("Failed to parse number");
    for i in 1..=n {
        if n % i == 0 {
            println!("{i}")
        }
    }
}
