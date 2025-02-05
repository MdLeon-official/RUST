use std::io;

fn main() {
    let mut inp_num = String::new();
    let mut store_vec: Vec<&str> = vec![];
    let mut sum = 0;

    io::stdin()
        .read_line(&mut inp_num)
        .expect("Failed to read input");
    let t: i32 = inp_num.trim().parse().expect("Failed to parse input");

    for _ in 0..t {
        let mut user_str = String::new();
        io::stdin()
            .read_line(&mut user_str)
            .expect("Failed to read input");
        let polyhedron = user_str.trim();

        sum += match polyhedron {
            "Tetrahedron" => 4,
            "Cube" => 6,
            "Octahedron" => 8,
            "Dodecahedron" => 12,
            "Icosahedron" => 20,
            _ => 0,
        };
    }

    println!("{}", sum);
}
