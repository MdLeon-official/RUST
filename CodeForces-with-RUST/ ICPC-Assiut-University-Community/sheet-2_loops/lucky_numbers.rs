use std::io;

fn main() {
    let mut inp_num = String::new();

    io::stdin()
        .read_line(&mut inp_num)
        .expect("Failed to read input");
    let num: Vec<i64> = inp_num
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to read input"))
        .collect();
    let a = num[0];
    let b = num[1];

    let mut output = String::new();
    let mut found = false;

    for i in a..=b {
        let to_str = i.to_string();
        if to_str.chars().all(|x| x == '4' || x == '7') {
            found = true;
            output.push_str(&to_str);
            output.push(' ');
        }
    }
    if found {
        println!("{}", output.trim())
    } else {
        println!("-1")
    }
}
