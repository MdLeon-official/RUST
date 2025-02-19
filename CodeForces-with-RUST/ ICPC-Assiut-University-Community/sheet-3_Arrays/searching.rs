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

    let mut third_inp = String::new();
    io::stdin()
        .read_line(&mut third_inp)
        .expect("Failed to read input");
    let third_num = third_inp.trim().parse().expect("Failed to read input");

    for (position, &num) in sec_vec.iter().enumerate() {
        if num == third_num {
            println!("{}", position);
            return;
        }
    }
    println!("-1");
}
