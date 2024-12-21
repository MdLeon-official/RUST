fn main() {
    // 3: SCOPE AND OWNERSHIP
    let age = 18;
    {
        let is_ok = false; // This variable exists here
    } // `is_ok` no longer exists

    // 4: THE COPY TRAIT
    let time = 2025;
    let year = time;
    println!("The {time} and the {year}.");

    // 5: THE STRING TYPE
    let text = String::new(); // Creates an empty string on the heap
    let candy = String::from("KitKat"); // Creates a `String` based on static data

    // 6: THE push_str METHOD ON A STRING TYPE
    let mut name = String::from("Leon");
    println!("Name before: {name}");
    name.push_str(" Islam");
    println!("Name after: {name}");

    // 7: MOVES AND OWNERSHIP
    let person = String::from("Leon");
    let genius = person; // Ownership moved to `genius`
    // println!("This is an error {person}"); // Error: `person` is invalid
    println!("This will run {genius}");

    // 8: THE DROP FUNCTION
    let a = String::from("hi");
    drop(a);
    // let b = a; // Error: `a` is already dropped

    // THE CLONE TRAIT
    let cln = String::from("Clone");
    let cln2 = cln.clone();
    println!("This is cloned {cln}");

    // REFERENCES AND BORROWING
    let my_stack_value = 2;
    let my_stack_reference = &my_stack_value;
    println!("{}", my_stack_reference);
    println!("{}", *my_stack_reference);

    let my_heap_value = String::from("String");
    let my_heap_reference = &my_heap_value;
    println!("{}", my_heap_reference);
    println!("{}", *my_heap_reference);

    // OWNERSHIP AND FUNCTION PARAMETERS
    let example = 23;
    print_my_i_val(example); // Ownership remains with `example`
    println!("{example} is still valid.");

    let example_str = String::from("This is a string.");
    print_my_s_val(example_str);
    // println!("{example_str} is not valid."); // Error: Ownership moved

    // MUTABLE PARAMETERS
    let ex_mp = String::from("Berger");
    mut_par(ex_mp);

    // RETURN VALUES
    let rt_ex = bake_cake();
    println!("I have a {rt_ex} cake.")
}

// OWNERSHIP AND FUNCTION PARAMETERS
fn print_my_i_val(value: i32) {
    println!("The value is {value}")
}
fn print_my_s_val(value: String) {
    println!("The value is {value}")
}

// MUTABLE PARAMETERS
fn mut_par(mut meal: String) {
    meal.push_str(" and Fries");
    println!("Example of mutable parameters: {meal}")
}

// RETURN VALUES
fn bake_cake() -> String {
    let cake = String::from("Chocolate");
    cake
}
