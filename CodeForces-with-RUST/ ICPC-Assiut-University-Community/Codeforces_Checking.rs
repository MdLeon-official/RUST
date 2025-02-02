use std::io;

fn main() {
    let mut inp = String::new();
    let cf = "codeforces";
    let cf_c: Vec<String> = cf.chars().map(|c| c.to_string()).collect();

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");
    let n: i64 = inp.trim().parse().expect("Failed");

    for _ in 0..n {
        let mut cases = String::new();
        io::stdin()
            .read_line(&mut cases)
            .expect("Failed to read input.");
        cases = cases.trim().to_string();
        if cf_c.contains(&cases) {
            println!("YES")
        }else {
            println!("NO")
        }
    }
}
