use std::io;

fn main() {
    let mut usr_inp = String::new();
    io::stdin().read_line(&mut usr_inp).expect("Failed");
    let mut numbers: Vec<i32> = usr_inp
        .trim()
        .split("+")
        .map(|x| x.parse().expect("Failed"))
        .collect();
    numbers.sort();

    let result = numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("+");
    println!("{}", result);
}
