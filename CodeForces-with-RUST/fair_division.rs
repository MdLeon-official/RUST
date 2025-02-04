use std::io;

fn main() {
    let mut inp_num = String::new();
    io::stdin()
        .read_line(&mut inp_num)
        .expect("Failed to read input");
    let t: i32 = inp_num.trim().parse().expect("Failed to parse input");

    for _ in 0..t {
        let mut inp_n = String::new();
        io::stdin()
            .read_line(&mut inp_n)
            .expect("Failed to read input");
        let n: usize = inp_n.trim().parse().expect("Failed to parse input");

        let mut inp_weights = String::new();
        io::stdin()
            .read_line(&mut inp_weights)
            .expect("Failed to read input");
        let weights: Vec<i32> = inp_weights
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse input"))
            .collect();

        let total_weight: i32 = weights.iter().sum();

        if total_weight % 2 != 0 {
            println!("NO");
            continue;
        }

        let count_1 = weights.iter().filter(|&&w| w == 1).count() as i32;
        let count_2 = weights.iter().filter(|&&w| w == 2).count() as i32;

        let target_weight = total_weight / 2;

        let used_2 = std::cmp::min(count_2, target_weight / 2);
        let remaining_weight = target_weight - (used_2 * 2);

        if remaining_weight <= count_1 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
