use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let nums: i32 = input.trim().parse().expect("Failed to read input");

    for i in 1..=nums {
        let a = "*".repeat(i as usize);
        println!("{a}")
    }
}
