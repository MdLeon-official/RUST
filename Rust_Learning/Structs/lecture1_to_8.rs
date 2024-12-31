// Lecture 1: Define the Struct
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn main() {
    // Lecture 2: Creating an Instance of a Struct in Rust
    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    // Lecture 3: Access Struct Fields
    println!(
        "My {} this morning cost {}. It is {} that it was hot",
        mocha.name, mocha.price, mocha.is_hot
    );

    let favourite_coffee = mocha.name;
    println!("Favourite coffee: {}", favourite_coffee);
    // println!("{}", mocha.name); // Error: ownership of `mocha.name` moved

    // Lecture 4: Overwrite Struct Fields
    let mut beverage = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };
    beverage.name = String::from("Caramel Macchiato");
    beverage.price = 6.99;
    beverage.is_hot = false;

    println!(
        "My {} this morning cost {}. It is {} that it was hot",
        beverage.name, beverage.price, beverage.is_hot
    );

    // Lecture 5: Create Structs in a Function
    let name5 = String::from("Latte");
    let coffee = make_coffee(name5, 4.99, true);

    println!(
        "My {} this morning cost {}. It is {} that it was hot",
        coffee.name, coffee.price, coffee.is_hot
    );

    // Lecture 6: Simplified Struct Creation Syntax
    let name = String::from("Latte");
    let price = 3.99;
    let is_hot = false;

    let latte = Coffee {
        name,
        price,
        is_hot,
    };

    println!(
        "My {} this morning cost {}. It is {} that it was hot",
        latte.name, latte.price, latte.is_hot
    );

    // Lecture 7: Struct Update Syntax
    let mut caramel_macchiato = Coffee {
        name: String::from("Caramel Macchiato"),
        ..mocha // Using fields from `mocha`
    };

    // Lecture 8: Passing Structs into a Function
    drink_coffee(&mut caramel_macchiato);
    println!("Updated price after drinking: {}", caramel_macchiato.price);
}

// Lecture 5: Create Structs in a Function
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

// Lecture 8: Passing Structs into a Function
fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.price = 10.99; // Update the price
}
