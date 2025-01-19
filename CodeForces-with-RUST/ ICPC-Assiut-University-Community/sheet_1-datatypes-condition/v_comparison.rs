use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");

    if inp.contains(">") == true {
        let gt: Vec<&str> = inp
            .trim()
            .split(">")
            .collect();
        let a: i64 = gt[0].trim().parse().expect("Failed to read input.");
        let b: i64 = gt[1].trim().parse().expect("Failed to read input.");
        if a > b {
            println!("Right")
        }
        else {
            println!("Wrong")
        }
    }
    else if inp.contains("<") == true {
        let gt: Vec<&str> = inp
            .trim()
            .split("<")
            .collect();
        let a: i64 = gt[0].trim().parse().expect("Failed to read input.");
        let b: i64 = gt[1].trim().parse().expect("Failed to read input.");
        if a < b {
            println!("Right")
        }
        else {
            println!("Wrong")
        }
    }
    else if inp.contains("=") == true {
        let gt: Vec<&str> = inp
            .trim()
            .split("=")
            .collect();
        let a: i64 = gt[0].trim().parse().expect("Failed to read input.");
        let b: i64 = gt[1].trim().parse().expect("Failed to read input.");
        if a == b {
            println!("Right")
        }
        else {
            println!("Wrong")
        }
    }
    
}
