use std::io;

fn main() {
    const PI:f64 = 3.141592653;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let inp:f64 = input.trim().parse().expect("Failed to read input.");
    println!("{}", PI * inp * inp);
}
