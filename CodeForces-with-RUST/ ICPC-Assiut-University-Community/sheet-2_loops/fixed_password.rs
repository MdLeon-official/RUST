use std::io;

fn main() {
    let mut correct: i64 = 1999;
    let mut cond = true;
    while cond {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read input.");
        let num: i64 = inp.trim().parse().expect("Failed to read input.");
        if num == correct {
            println!("Correct");
            cond = false;
        } else {
            println!("Wrong")
        }
    }
}
