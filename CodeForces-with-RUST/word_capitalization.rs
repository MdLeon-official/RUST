use std::io;

fn main() {
    let mut strings = String::new();
    io::stdin().read_line(&mut strings).expect("Failed to read input");
    let mut v: Vec<char> = strings.trim().chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    for i in v {
        print!("{i}")
    }
    print!("\n")
}
