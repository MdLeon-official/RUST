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

        if no_use.trim().to_lowercase() == "yes" {
            println!("YES")
        } else {
            println!("NO")
        }
    }
}
