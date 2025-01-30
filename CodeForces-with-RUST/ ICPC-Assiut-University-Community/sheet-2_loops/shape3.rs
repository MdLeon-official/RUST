use std::io;

fn main() {
    let mut inp = String::new();

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");

    let n: i64 = inp.trim().parse().expect("Failed");

    for i in 1..=n {
        let spaces = " ".repeat((n - i).try_into().unwrap());
        let stars = "*".repeat((2 * i - 1).try_into().unwrap());
        println!("{}{}", spaces, stars);
    }

    for i in (1..=n).rev() {
        let spaces = " ".repeat((n - i).try_into().unwrap());
        let stars = "*".repeat((2 * i - 1).try_into().unwrap());
        println!("{}{}", spaces, stars);
    }
}
