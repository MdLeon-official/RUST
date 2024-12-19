/*

Declare a `is_concert` variable set to a boolean.
Declare a `is_event` variable assigned to `is_concert`.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.

Declare a `sushi` variable to set to a string literal of "Salmon"
Declare a `dinner` variable assigned to the `sushi` variable.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.

Repeat the previous example but use a heap String instead.
Will Rust move ownership? Explain why the result is different
from the previous operation.

The `clear` method modifies a heap String to have no content.
Declare an `eat_meal` function that accepts a `meal` parameter
of type String. In the body of `eat_meal`, invoke the `clear`
method on the `meal` parameter.

In the `main` function, invoke the `eat_meal` function and pass
in your "Salmon" String. Explain what happens when the eat_meal
function runs. Describe the complete movement of ownership of
the "Salmon" String throughout the program.

Say we want to keep the String around after `eat_meal` is
called. How can we continue to have access to the String in
the `main` function? Print out the (empty) String.
*/

fn main() {
    //1
    // For simple types like booleans, Rust will copy instead of move
    let is_concert = true;
    let is_event = is_concert;
    // We can print both because boolean was copied, not moved
    println!("is_concert: {}", is_concert);
    println!("is_event: {}", is_event);
    /*Rust didn't move ownership here because booleans implement the Copy trait, which means their values are simply copied
    when assigned to a new variable. This is different from complex types like Strings that get moved instead.*/

    //2
    let sushi = "Salmon";
    let dinner = sushi;
    // String literals are stored in program memory and have the Copy trait
    // So both variables can be used since the value is copied, not moved
    println!("The sushi variable still contains: {}", sushi);
    println!("The dinner variable contains: {}", dinner);
    /* Explanation: With string literals like "Salmon", Rust doesn't move ownership Instead, it makes a copy because string literals:
    1. Are stored directly in the program code
    2. Have a fixed size known at compile time
    3. Implement the Copy trait automatically
    This means we can keep using both sushi and dinner variables. No ownership moving happens with string literals!
    */

    //3
    let meat = String::from("Beef");
    let lunch = meat; // ownership moves from meat to lunch
    println!("This is valid: {lunch}");
    // println!("This is not valid: {meat}");
    /* Unlike the string literal example, heap Strings get moved instead of copied
    because they:
    1. Are stored on the heap with dynamic size
    2. Don't implement the Copy trait
    3. Can only have one owner at a time
    So after assigning meat to lunch, meat is no longer valid
    */

    //4, 5, 6
    let salmon = String::from("Salmon");
    // Ownership of the "Salmon" String moves from salmon variable to meal parameter
    // When eat_meal runs, it clears the String content then meal parameter goes out of scope
    eat_meal(salmon.clone());
    // Can't use salmon variable here because ownership was moved to eat_meal function
    // print!("{salmon}"); //ERROR
    println!("Salmon now can be used because we used \"clone()\" : {salmon}")
}

//4
fn eat_meal(mut meal: String) {
    println!("This is meal before clearing: {meal}");
    meal.clear();
    println!("This is meal after clearing: {meal}");
}
