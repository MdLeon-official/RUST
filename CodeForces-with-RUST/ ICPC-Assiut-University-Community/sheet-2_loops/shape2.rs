use std::io;

fn main() {
    let mut inp = String::new();

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");

    let mut n: i64 = inp.trim().parse().expect("Failed");

    for i in 1..=n {
        let space = " ".repeat((n - i) as usize);
        let num = "*".repeat((2 * i - 1) as usize);
        println!("{}{}", space, num);
    }
}
