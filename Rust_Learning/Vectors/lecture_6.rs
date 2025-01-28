fn main() {
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let mut pizza_toppings = vec![pepperoni, mushroom, sausage];

    pizza_toppings[1] = String::from("Olives");
    println!("{:?}", pizza_toppings);

    let target_toppings = &mut pizza_toppings[2];
    target_toppings.push_str(" and Meatballs");

    let another_topping = &pizza_toppings[2];
    println!("{:?}", pizza_toppings);
    println!("{:?}", another_topping);
}
