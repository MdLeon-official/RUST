use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let inp: i64 = input.trim().parse().expect("Failed to read input");
    let sum = (inp * (inp + 1)) / 2;
    println!("{}", sum);
}
