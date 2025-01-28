fn main() {
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];
    let mut delicious_toppings = pizza_toppings;

    let topping_reference = &delicious_toppings[1];
    println!("The topping is {}", topping_reference);

    delicious_toppings.push(String::from("Olives"));

    // println!("The topping is {}", topping_reference);
}
