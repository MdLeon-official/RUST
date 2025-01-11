use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let mut format = input.trim().chars().next().unwrap();
    if format.is_ascii_digit() == true {
        println!("IS DIGIT");
    }else {
        println!("ALPHA");
        if format.is_uppercase() == true {
            println!("IS CAPITAL");
        }else {
            println!("IS SMALL");
        }
    }
}
