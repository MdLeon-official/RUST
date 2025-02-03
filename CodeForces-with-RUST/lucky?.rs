use std::io;

fn main() {
    let mut inp = String::new();

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");
    let n: i64 = inp.trim().parse().expect("Failed");

    for _ in 0..n {
        let mut cases = String::new();
        let mut sum1 = 0;
        let mut sum2 = 0;

        io::stdin()
            .read_line(&mut cases)
            .expect("Failed to read input.");
        let cf_c: Vec<String> = cases.trim().chars().map(|c| c.to_string()).collect();

        for i in 0..3 {
            let str_to_int: i64 = cf_c[i].parse().expect("Failed to read input");
            sum1 += str_to_int;
        }
        for i in 3..6 {
            let str_to_int: i64 = cf_c[i].parse().expect("Failed to read input");
            sum2 += str_to_int;
        }
        if sum1 == sum2 {
            println!("YES")
        } else {
            println!("NO")
        }
    }
}
