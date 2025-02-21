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

    let result: Vec<String> = sec_vec
        .iter()
        .map(|&i| {
            if i > 0 {
                "1".to_string()
            } else if i < 0 {
                "2".to_string()
            } else {
                "0".to_string()
            }
        })
        .collect();

    println!("{}", result.join(" "));
}
