use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let my_int: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to read input"))
        .collect();

    let num0: f64 = my_int[0];
    let num1: f64 = my_int[1];

    if num0 == 0.0 && num1 == 0.0 {
        println!("Origem")
    } else if num0 == 0.0 {
        println!("Eixo Y")
    } else if num1 == 0.0 {
        println!("Eixo X")
    } else if num0 > 0.0 && num1 > 0.0 {
        println!("Q1")
    } else if num0 > 0.0 && num1 < 0.0 {
        println!("Q4")
    } else if num0 < 0.0 && num1 > 0.0 {
        println!("Q2")
    } else if num0 < 0.0 && num1 < 0.0 {
        println!("Q3")
    }
}
