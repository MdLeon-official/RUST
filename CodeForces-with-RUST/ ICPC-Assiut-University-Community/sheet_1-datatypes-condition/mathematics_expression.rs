use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");

    if inp.contains("=") == true {
        let check: Vec<&str> = inp
            .trim()
            .split("=")
            .collect();
        let part_one = check[0];
        let part_two: i64 = check[1].trim().parse().expect("Failed to read input.");
        
        if part_one.contains("+") {
            let split_by: Vec<&str> = part_one
            .trim()
            .split("+")
            .collect();
            
            let a: i64 = split_by[0].trim().parse().expect("Failed to read input.");
            let b: i64 = split_by[1].trim().parse().expect("Failed to read input.");
            if (a+b) == part_two {
                println!("Yes");
            }else {
                println!("{}", a+b);
            }
        }
        else if part_one.contains("-") {
            let split_by: Vec<&str> = part_one
            .trim()
            .split("-")
            .collect();
            
            let a: i64 = split_by[0].trim().parse().expect("Failed to read input.");
            let b: i64 = split_by[1].trim().parse().expect("Failed to read input.");
            if (a-b) == part_two {
                println!("Yes");
            }else {
                println!("{}", a-b);
            }
        }
        else if part_one.contains("*") {
            let split_by: Vec<&str> = part_one
            .trim()
            .split("*")
            .collect();
            
            let a: i64 = split_by[0].trim().parse().expect("Failed to read input.");
            let b: i64 = split_by[1].trim().parse().expect("Failed to read input.");
            if (a*b) == part_two {
                println!("Yes");
            }else {
                println!("{}", a*b);
            }
        }
    }

    
    
}
