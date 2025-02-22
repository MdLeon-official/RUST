use std::io;

fn main() {
    let mut first_inp = String::new();
    io::stdin()
        .read_line(&mut first_inp)
        .expect("Failed to read input");

    let mut second_inp = String::new();
    io::stdin()
        .read_line(&mut second_inp)
        .expect("Failed to read input");

    let sec_vec: Vec<i64> = second_inp
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to read input"))
        .collect();

    for (i, &num) in sec_vec.iter().enumerate() {
        if num <= 10 {
            println!("A[{}] = {}", i, num);
        }
    }
}
