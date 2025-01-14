use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let mut days: i64 = input.trim().parse().expect("Failed to read input.");
    let year = days / 365;
    println!("{} years", year);
    days = days % 365;
    let month = days / 30;
    println!("{} months", month);
    days = days % 30;

    println!("{} days", days);
}
