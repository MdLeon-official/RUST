fn main() {
    //1. Immutable and Mutable Reference Parameters
    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);

    //2. Multiple Immutable References
    /*
    Reference: Allows accessing data without copying it.
    Problem in Other Languages: Multiple references can cause bugs if one changes the data unexpectedly.
    Rust’s Rules:
    Many immutable references are allowed simultaneously (safe to share).
    Only one mutable reference is allowed at a time (avoids conflicts).
     */
    let car = String::from("Red");
    let ref1 = &car;
    let ref2 = &car;
    println!("Ref1: {}, Ref2: {}", ref1, ref2);


    //3. Mutable Reference Restrictions
    /*
    -->Immutable References (&value):
    You can create multiple references that only read the value. They cannot change the value.
    -->Mutable References (&mut value):
    Only one mutable reference is allowed at a time. It can change the value.
    -->Why Only One Mutable Reference?
    If a mutable reference changes the value, other references (even immutable ones) might get unexpected or wrong data.
    Rust prevents this to keep your program safe and consistent.
    -->Lifetimes Matter:
    A reference is only valid until it’s last used in the code. After that, you can create new references safely.
    -->Rules:
    Many immutable references? ✅ Fine. One mutable reference? ✅ Fine. Mixing mutable and immutable references at the same time? ❌ Not allowed.
     */
    let mut book = String::from("Rust Programming"); // A mutable value
    // Create a mutable reference to 'book'
    let mut_ref = &mut book; // This reference can change the value
    mut_ref.push_str(" for Beginners"); // Modify the value
    println!("Changed by mut_ref: {}", mut_ref); // Prints: "Rust Programming for Beginners"
    // Create an immutable reference after the mutable reference is no longer used
    let immut_ref = &book; // This reference can only read the value
    println!("Read by immut_ref: {}", immut_ref); // Prints: "Rust Programming for Beginners"
    // Rules ensure safety: mut_ref and immut_ref never coexist at the same time.


    //4. Ownership with Immutable and Mutable References
    /*
    Immutable References (&)
        References that cannot change the value they point to.
    Special Feature: Copy Trait
        Immutable references can be copied, meaning you can create multiple references to the same value. 
        This is safe because immutable references only read the value; they cannot modify it.
    */
    let coffee = String::from("Mocha"); // A string value
    let a = &coffee; // Immutable reference
    let b = a; // Copy of the immutable reference
    println!("a: {}", a); // Output: Mocha
    println!("b: {}", b); // Output: Mocha
    // Both a and b can read the value because they are independent, safe copies of the same reference.
    /*
    Mutable References (&mut)
        References that can change the value they point to.
    Special Rule: No Copy Trait
        Mutable references cannot be copied. If you assign a mutable reference to another variable,
        ownership of the reference moves, and the original reference becomes invalid.
    */
    let mut coffee = String::from("Latte"); // A mutable string
    let a = &mut coffee; // Mutable reference
    let b = a; // Ownership moves to 'b'
    b.push_str(" with Milk"); // Modify the value
    println!("b: {}", b); // Output: Latte with Milk
    // Using 'a' now will cause an error because 'b' owns the reference
    // println!("a: {}", a); // Error: 'a' is invalid



    //5. Dangling References
    /*
    ==> What is a Dangling Reference?
    A dangling reference happens when a reference (pointer) points to a place in memory that no longer holds valid data.
    Imagine you have an address to a house, but the house is no longer there. The address is still valid, but there's nothing there.
    ==> How Rust Prevents Dangling References
    Rust uses a system called ownership to make sure that references to data are always safe.
    When a piece of data goes out of scope (like when a function ends), it’s removed from memory, and any reference to that data becomes invalid.
    Rust makes sure that we can’t use a reference to data after it has been removed from memory, avoiding dangling references.
    */
    /*
    The city string is created inside the function, but it will be deleted when the function finishes.
    Returning a reference to city is a problem because once the function ends, city no longer exists.

    ==> Why is this a Problem?
    After the function ends, the memory for city is freed.
    If we try to use the reference to city, it would point to nothing, causing a dangling reference (an invalid reference).
    ==> How Rust Catches This
    Rust’s compiler will catch this mistake and give an error message:
        "This function's return type contains a borrowed value, but there is no value for it to be borrowed from."
    This tells us that we can’t return a reference to something that no longer exists.
    */
    create_city();


    //6. Ownership with Arrays and Tuples
    /*
    ==> What is Ownership in Collections?
    In Rust, ownership means deciding who is responsible for cleaning up data.
    Collection types like arrays and tuples can hold multiple values. They also have ownership, meaning they own their elements.
    ==> Arrays and Ownership
    If an array holds simple types like booleans, the ownership rules are simple:
        The array owns its elements.
        If you extract an element from the array, Rust copies the value because booleans implement the Copy trait.
    */
    let registrations = [true, false, true]; // 'registrations' owns the array and its elements.
    let first = registrations[0]; // 'first' gets a copy of the boolean.
    /*
    Arrays with Heap Data
    When an array holds heap data (like String), ownership works differently. 
    The array owns the String, but since String doesn't implement the Copy trait, Rust will move ownership when you extract an element.
    */
    let languages = ["Rust".to_string(), "JavaScript".to_string()];
    let first = languages[0]; // Error! We cannot move out of the array.
    // Why the error? The String inside the array will be moved to first, leaving the array in a partial state of ownership.
    // How to Avoid Ownership Move?
    // Clone the Data: You can clone (copy) the value instead of moving it:
    let first = languages[0].clone(); // Clones the String.
    // This creates a duplicate of the String instead of moving it.
    // Borrow the Data: Instead of moving the ownership, you can borrow a reference to the element:
    let first = &languages[0]; // Borrows a reference to the String.
    // This allows the array to remain the owner, and you can still use the data.

    /*
    Tuples and Ownership
    Tuples work similarly to arrays. They can hold different types of data, and ownership follows the same rules.
    To access tuple elements, we use dot notation.
    */
    let languages_tuple = ("Rust".to_string(), "JavaScript".to_string());
    let first = &languages_tuple.0; // Borrows a reference to the first element.
/*
Key Points to Remember:
    Arrays and tuples own their elements.
    For simple types (like booleans), Rust copies the value when extracted.
    For heap data types (like String), Rust moves ownership when extracted.
    You can clone data to make a copy, or you can borrow a reference to avoid moving ownership.
*/

}

//1. Immutable and Mutable Reference Parameters
/*
There are 4 possible options for how we can define this parameter.
i. meal: String = This means we are defining a parameter that will take full ownership of the String, but cannot modify it
ii. mut meal: String = This means the 'meal' parameter will take ownership over the String and can modify it
iii. meal: &String = This means 'meal' is a reference (memory address) to a String, but cannot modify it
iv. meal: &mut String = This means we get a mutable reference that can read and modify the String at that address
*/
fn add_flour(meal: &mut String) {
    meal.push_str("Add flour"); // Can modify because we have a &mut reference
}
fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}"); // Only needs to read, so regular & reference is fine
}


//2. Multiple Immutable References
// ..
// ..

//5. Dangling References
fn create_city() -> String {
    // let city = String::from("New York");
    // &city // Trying to return a reference to `city`

    String::from("New York") // Return the String itself, not a reference => thats how to fix this
    // This works because we’re returning the data itself, not a reference. Rust transfers ownership of the data, so there’s no dangling reference.
    
}
