fn main() {
    // Array example with Debug and Display traits
    let values = ["Hello", "world"];

    // This will not compile because arrays don't implement the Display trait
    // println!("{}", values);

    // Using Debug trait
    println!("{:?}", values); // Debug representation
    println!("{:#?}", values); // Pretty-printed Debug representation

    // Struct example
    #[derive(Debug)] // Derive the Debug trait for the struct
    struct Coffee {
        name: String,
        price: f64,
        is_hot: bool,
    }

    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    // This will not compile because the struct doesn't implement Display
    // println!("{}", mocha);

    // Using Debug representation for the struct
    println!("{:?}", mocha); // Debug representation
    println!("{:#?}", mocha); // Pretty-printed Debug representation

    // Example of an unused variable warning
    // #[allow(unused_variables)] // Uncomment to suppress the unused variable warning
    let unused_coffee = Coffee {
        name: String::from("Latte"),
        price: 3.99,
        is_hot: false,
    };
}
