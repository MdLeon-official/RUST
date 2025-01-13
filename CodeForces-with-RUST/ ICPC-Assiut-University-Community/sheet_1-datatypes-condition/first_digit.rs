use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let v: String = input
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();

    let my_int: i32 = format!("{}", v.chars().nth(0).unwrap())
        .parse()
        .unwrap();

    if my_int % 2 == 0 {
        println!("{}", "EVEN")
    }else{
        println!("{}", "ODD")
    }
}
