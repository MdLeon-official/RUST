    use std::io;
     
    fn main() {
        let mut input = String::new();
     
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
     
        let last_digit_sum: u64 = input
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<u64>().ok())
            .map(|num| num % 10)
            .sum();
     
        println!("{}", last_digit_sum);
    }
