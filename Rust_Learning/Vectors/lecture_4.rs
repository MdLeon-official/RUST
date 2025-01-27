fn main() {
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    let option: Option<&String> = pizza_toppings.get(50);
    match option {
        Some(topping) => println!("The topping is {topping}"),
        None => println!("No value at that index position."),
    }
}
