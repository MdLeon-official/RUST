fn main() {
    //2
    let action_hero = String::from("Arnold Schwarzenegger");
    // Borrow the first name
    let first_name = &action_hero[0..6]; // "Arnold"
                                         // Borrow the last name
    let last_name = &action_hero[7..21]; // "Schwarzenegger"
                                         // Print them
    println!("First name: {}", first_name);
    println!("Last name: {}", last_name);

    //3
    let action_hero = "Arnold Schwarzenegger"; // This is a string literal.
    let first_name = &action_hero[0..6]; // This is a slice of the first 6 characters.
    println!("{}", first_name); // This will print "Arnold".

    //4
    let food = "🍕pizza"; // A string slice with an emoji.
    println!("Length of 'food': {}", food.len()); // Prints 8 bytes because emoji takes 4 bytes.
                                                  // Correct slicing:
    let pizza_slice = &food[0..4]; // This captures the full emoji.
    println!("{}", pizza_slice); // Prints the full emoji: 🍕

    //5
    let name = String::from("Arnold Schwarzenegger");
    // Omitting start index
    let part1 = &name[..6]; // "Arnold"
                            // Omitting end index
    let part2 = &name[7..]; // "Schwarzenegger"
                            // Slicing the entire string
    let full_name = &name[..]; // "Arnold Schwarzenegger"
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Full Name: {}", full_name);

    //6
    let action_hero = String::from("Arnold Schwarzenegger");
    let another_action_hero = "Sylvester Stallone";
    // Using a String reference (&String)
    do_hero_stuff(&action_hero);
    // Using a string slice (&str)
    do_hero_stuff(another_action_hero);
    // The function do_hero_stuff accepts both &String (via deref coercion) and &str.
    // The code works because deref coercion automatically converts the &String to a &str, allowing the function to work with both types.

    //7
    let values = [4, 8, 15, 16, 23, 42];
    // Create a slice from the first three elements
    let my_slice = &values[0..3];
    println!("{:?}", my_slice); // Output: [4, 8, 15]
                                // Create a slice from the middle elements
    let middle_slice = &values[2..4];
    println!("{:?}", middle_slice); // Output: [15, 16]
                                    // Create a slice from index 2 to the end of the array
    let end_slice = &values[2..];
    println!("{:?}", end_slice); // Output: [15, 16, 23, 42]
                                 // Create a full array reference (with fixed length)
    let full_ref: &[i32; 6] = &values;
    println!("{:?}", full_ref); // Output: [4, 8, 15, 16, 23, 42]

    //8
    let values = [4, 8, 15, 16, 23, 42];
    // Regular reference to the full array
    let regular_reference = &values;
    // Slice of the first three elements
    let slice_of_three = &values[0..3];
    // Passing full array reference to the print_length function
    print_length(regular_reference); // Output: Length: 6
                                     // Passing array slice to the print_length function
    print_length(slice_of_three); // Output: Length: 3

    //9
    // Declare a mutable array of integers
    let mut my_array = [10, 15, 20, 25, 30];
    // Borrow a mutable slice of the array, targeting index 2 up to (but not including) index 4
    let mut my_slice = &mut my_array[2..4]; // Mutable slice
                                            // Print the original slice
    println!("Original slice: {:?}", my_slice); // Output: [20, 25]
                                                // Modify an element in the mutable slice
    my_slice[0] = 100;
    // Print the modified slice
    println!("Modified slice: {:?}", my_slice); // Output: [100, 25]
                                                // Print the entire array to show that the original array is affected
    println!("Original array after mutation: {:?}", my_array); // Output: [10, 15, 100, 25, 30]
}

//6
fn do_hero_stuff(hero_name: &str) {
    println!("{} saves the day", hero_name);
}

//8
fn print_length(reference: &[i32]) {
    println!("Length: {}", reference.len());
}
/*
Explanation:

    The function print_length accepts an array slice &[i32]. This allows it to work with both full array references and array slices.
    When passing regular_reference (a reference to the whole array), Rust automatically coerces it into a slice, and the function works without issues.
    When passing slice_of_three (an array slice), the function also works as expected because it is already in the required format (&[i32]).

This approach is more flexible and versatile than defining a function that only accepts a reference to a
specific array length (e.g., &[i32; 6]), making it easier to work with both slices and full arrays.
*/
