use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");

    let n: i32 = inp.trim().parse().expect("Failed to parse number");
    let rev = n.to_string().chars().rev().collect::<String>();
    let rev_num: i32 = rev.parse().expect("Failed to parse reversed number");

    println!("{}", rev_num);
    if n == rev_num {
        println!("YES");
    } else {
        println!("NO");
    }
}
