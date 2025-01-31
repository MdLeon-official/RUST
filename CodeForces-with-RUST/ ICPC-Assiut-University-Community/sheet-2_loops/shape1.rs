use std::io;

fn main() {
    let mut inp = String::new();

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");

    let mut n: i64 = inp.trim().parse().expect("Failed");

    while n > 0 {
        let num = "*".repeat(n as usize);
        println!("{}", num);
        n -= 1;
    }
}
