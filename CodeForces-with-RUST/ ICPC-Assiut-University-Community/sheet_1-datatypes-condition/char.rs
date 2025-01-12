use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read message.");
    let inp_check = input.trim().chars().next().unwrap();
    if inp_check.is_uppercase() == true {
        println!("{}", inp_check.to_lowercase());
    }
    if inp_check.is_lowercase() == true {
        println!("{}", inp_check.to_uppercase());
    }
}
