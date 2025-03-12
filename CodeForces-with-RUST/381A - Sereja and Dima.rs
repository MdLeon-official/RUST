use std::io;

fn main() {
    let mut first_inp = String::new();
    io::stdin()
        .read_line(&mut first_inp)
        .expect("Failed to read line");

    let mut user_sp = String::new();
    io::stdin()
        .read_line(&mut user_sp)
        .expect("Failed to read line");
    let mut number: Vec<i64> = user_sp
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    let mut sereja_score = 0;
    let mut dima_score = 0;
    let mut turn = 0;

    while !number.is_empty() {
        let selected;
        if number.first() >= number.last() {
            selected = number.remove(0);
        } else {
            selected = number.pop().unwrap();
        }

        if turn % 2 == 0 {
            sereja_score += selected;
        } else {
            dima_score += selected;
        }
        turn += 1;
    }

    println!("{} {}", sereja_score, dima_score);
}
