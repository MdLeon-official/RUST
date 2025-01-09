    fn main() {
        let mut a = String::new();
        std::io::stdin()
            .read_line(&mut a)
            .expect("Failed to read line");
        let a: i32 = a.trim().parse().expect("Please enter a valid number");
     
        if a % 2 == 0 {
            if a == 2 {
                println!("no");
            } else {
                println!("yes");
            }
        } else {
            println!("no");
        }
    }

