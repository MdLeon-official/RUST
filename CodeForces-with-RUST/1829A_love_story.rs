use std::io;

fn main() {
    let str = "codeforces";
    let all: Vec<String> = str.chars().map(|c| c.to_string()).collect();

    let mut user = String::new();
    io::stdin()
        .read_line(&mut user)
        .expect("Failed to read input.");
    let n: i32 = user.trim().parse().expect("Failed to read input");

    for _ in 0..n {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read input.");
        let user_inp_vec: Vec<String> = inp.trim().chars().map(|c| c.to_string()).collect();
        let mut diff_count = 0;
        for i in 0..10 {
            if all[i] != user_inp_vec[i] {
                diff_count += 1;
            }
        }
        println!("{}", diff_count);
    }
}
