use std::io;

fn main() {
    let mut user_shape = String::new();
    io::stdin()
        .read_line(&mut user_shape)
        .expect("Failed to read input");

    let num = user_shape.trim().parse().expect("Failed to read input");

    for _ in 0..num {
        let mut main_code = String::new();
        io::stdin()
            .read_line(&mut main_code)
            .expect("Failed to read input");
        let mut main_vec: Vec<char> = main_code.trim().chars().collect();
        main_vec.reverse();
        let rev_vec = main_vec; 
        for item in rev_vec {
            print!("{} ", item);
        }
        print!("\n");
    }
}


