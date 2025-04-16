use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse().unwrap();
    let mut ans = 0;

    for _ in 0..n {
        let mut usr_inp = String::new();
        io::stdin().read_line(&mut usr_inp).unwrap();
        let num: Vec<i32> = usr_inp
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if num[0] + 2 <= num[1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
