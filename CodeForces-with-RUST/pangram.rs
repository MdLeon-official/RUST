use std::io;

fn main() {
    let mut inp_num = String::new();
    io::stdin().read_line(&mut inp_num).expect("Failed to read input");
    let mut inp_char = String::new();
    io::stdin().read_line(&mut inp_char).expect("Failed to read input");
    let mut count = 0;
    let check_ = inp_char.to_lowercase();

    for i in 'a'..='z' {
        if check_.contains(i) == true {
            count += 1;
        }
    }
    if count == 26 {
        println!("YES")
    }else{
        println!("NO")
    }
}
