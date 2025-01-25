use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input.");
    let num: i64 = inp.trim().parse().expect("Failed to read input.");
    prime(num);
}
fn prime(n: i64) {
    if n <= 1 {
        println!("NO");
        return;
    }

    let mut is_prime = true;
    for i in 2..n {
        if n % i == 0 {
            is_prime = false;
            break;
        }
    }

    if is_prime {
        println!("YES");
    } else {
        println!("NO");
    }
}
