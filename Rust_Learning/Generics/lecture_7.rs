enum CheeseSteak<T> {
    Plain,
    Topping(T),
}

fn main() {
    let mushroom = CheeseSteak::Topping("mushroom");
    let onions = CheeseSteak::Topping("onions".to_string());
    let topping = "pin".to_string();
    let pin = CheeseSteak::Topping(&topping);

    let mut plain: CheeseSteak<String> = CheeseSteak::Plain;
    // plain = CheeseSteak::Topping("sausage");
    plain = CheeseSteak::Topping("sausage".to_string());
}
