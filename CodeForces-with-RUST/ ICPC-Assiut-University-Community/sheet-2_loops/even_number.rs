use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let num: i64 = input
        .trim()
        .parse()
        .expect("Please provide a valid integer");

    let mut has_even = false;
    for i in 1..=num {
        if i % 2 == 0 {
            println!("{}", i);
            has_even = true;
        }
    }

    if !has_even {
        println!("-1");
    }
}
