use std::io;

fn main() {
    let mut inp = String::new();
    let mut count = 0;
    
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");
    
    let numbers: Vec<i64> = inp
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Please provide a valid input"))
        .collect();
    
    let k = numbers[0];
    let s = numbers[1];

    for x in 0..=k {
        for y in 0..=k {
            let z = s - x - y;
            if z >= 0 && z <= k {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
