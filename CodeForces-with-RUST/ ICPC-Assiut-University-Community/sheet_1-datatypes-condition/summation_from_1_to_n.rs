use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let inp: i64 = input.trim().parse().expect("Failed to read input");
    let mut sum: i64 = 0;

    for i in 0..=inp {
        sum = sum + i;
    }
    println!("{}", sum);
}
