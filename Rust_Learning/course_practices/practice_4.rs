
    /*
    Define an apply_to_jobs function that accepts a
    'number' parameter (an i32) and a 'title' parameter
    (a string). It should print out the string:
    "I'm applying to {number} {title} jobs".
     
    Example:
    apply_to_jobs(35, "Rust Developer")
    -> "I'm applying to 35 Rust Developer jobs"
     
    Define an is_even function that accepts a 'number'
    parameter (an i32). The function should return a true
    if the number is even and a false if the number is
    odd.
    Examples:
    is_even(8) -> true
    is_even(9) -> false
     
    Define an alphabets function that accepts a 'text'
    parameter (an &str). The function should return a
    tuple of two Booleans. The first Boolean should check
    if the text contains the letter 'a'. The second
    Boolean should check if the text contains the letter
    'z'. You can use the 'contains' method to check if a
    string contains a specific character. See the documentation:
    https://doc.rust-lang.org/std/primitive.str.html#method.contains
     
    Examples:
    println!("{:?}", alphabets("aardvark")); -> (true, false)
    println!("{:?}", alphabets("zoology"));  -> (false, true)
    println!("{:?}", alphabets("zebra"))09;    -> (true, true)
    */
    
fn main() {
    println!("Hello, world!");
    apply_to_jobs(8, "IT");

    if is_even(7) {
        println!("This is a even number.");
    }
    else{
        println!("This is a odd number.");
    }

    println!("{:?}", alphabets("aardvark"));
    println!("{:?}", alphabets("zoology"));
    println!("{:?}", alphabets("zebra"));
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} {title} jobs")
}

fn is_even(number: i32) -> bool {
    if number % 2 == 0 {
        return true;
    }else {
        return false;
    }
}

fn alphabets(text: &str) -> (bool,bool) {
    let contains_a = text.contains("a");
    let contains_z = text.contains("z");
    (contains_a, contains_z)
}
