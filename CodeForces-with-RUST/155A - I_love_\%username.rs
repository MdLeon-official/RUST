use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let mut points = String::new();
    io::stdin().read_line(&mut points).unwrap();

    let single_points: Vec<i32> = points
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let length = single_points.len();
    let mut result = 0;

    for i in 1..length {
        let mut is_max = true;
        let mut is_min = true;

        for j in 0..i {
            if single_points[j] >= single_points[i] {
                is_max = false;
            }
            if single_points[j] <= single_points[i] {
                is_min = false;
            }
        }

        if is_max || is_min {
            result += 1;
        }
    }

    println!("{}", result);
}
