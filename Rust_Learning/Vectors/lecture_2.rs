fn main() {
    let mut pizza_diameters = vec![5, 7, 34, 23, 4];
    pizza_diameters.push(16);
    pizza_diameters.push(18);
    println!("{:?}", pizza_diameters);

    pizza_diameters.insert(0, 2);
    println!("{:?}", pizza_diameters);

    let last_pizza_diameters = pizza_diameters.pop();
    println!(
        "The last pizza diameter is {}",
        last_pizza_diameters.unwrap()
    );
    println!("The last pizza diameter is {:?}", last_pizza_diameters);
    println!("{:?}", pizza_diameters);

    let third_diameter_from_start = pizza_diameters.remove(2);
    println!("third_diameter_from_start {:?}", third_diameter_from_start);
    println!("{:?}", pizza_diameters);
}
