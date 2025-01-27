fn main() {
    let mut pizza_diameters = vec![5, 7, 34, 23, 4];

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    let value = &pizza_toppings[2];
    let pizza_slice = &pizza_toppings[1..];
    // let value = &pizza_toppings[100];
    println!("{value}");
    println!("{pizza_slice:?}");
}
