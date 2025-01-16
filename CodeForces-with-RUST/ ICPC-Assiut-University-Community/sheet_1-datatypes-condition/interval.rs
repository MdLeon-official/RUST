    use std::io;
     
    fn main() {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read input");
        let x: f64 = inp.trim().parse().expect("Failed to get int.");
     
        if x >= 0 as f64 && x <= 25 as f64 {
            println!("Interval [0,25]");
        } else if x > 25 as f64 && x <= 50 as f64 {
            println!("Interval (25,50]");
        } else if x > 50 as f64 && x <= 75 as f64 {
            println!("Interval (50,75]");
        } else if x > 75 as f64 && x <= 100 as f64 {
            println!("Interval (75,100]");
        } else {
            println!("Out of Intervals");
        }
    }
