use std::io;

fn main() {
    let mut inp_num = String::new();

    io::stdin()
        .read_line(&mut inp_num)
        .expect("Failed to read input");
    let t: i32 = inp_num.trim().parse().expect("Failed to parse input");

    for _ in 0..t {
        let mut no_use = String::new();
        io::stdin()
            .read_line(&mut no_use)
            .expect("Failed to read input");

        let mut main_use = String::new();
        io::stdin()
            .read_line(&mut main_use)
            .expect("Failed to read input");
        let mut numbers: Vec<i64> = main_use
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to read input"))
            .collect();

        let mut max_product = 1;
        let mut min_index = 0;
        for (i, &num) in numbers.iter().enumerate() {
            if num < numbers[min_index] {
                min_index = i;
            }
        }
        numbers[min_index] += 1;

        for num in numbers {
            max_product *= num;
        }
        println!("{}", max_product);
    }
}
