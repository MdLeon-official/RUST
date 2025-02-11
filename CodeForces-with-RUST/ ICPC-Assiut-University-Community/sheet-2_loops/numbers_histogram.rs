use std::io;

fn main() {
    let mut user_shape = String::new();
    io::stdin()
        .read_line(&mut user_shape)
        .expect("Failed to read input");

    let mut user_x = String::new();
    io::stdin()
        .read_line(&mut user_x)
        .expect("Failed to read input");

    let mut user_main = String::new();
    io::stdin()
        .read_line(&mut user_main)
        .expect("Failed to read input");
    let nums: Vec<i64> = user_main
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to read"))
        .collect();

    for i in nums {
        let shapes: String = user_shape.trim().repeat(i as usize);
        println!("{}", shapes);
    }
}
