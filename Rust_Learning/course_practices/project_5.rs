    /*
    Define a `color_to_number` function that accepts a 'color'
    parameter (a string). Use if, else if, and else
    statements to return a corresponding numeric value based
    on the following rules:
    1. If the color is "red", return 1.
    2. If the color is "green", return 2.
    3. If the color is "blue", return 3.
    4. If the color is any other string, return 0.
     
    Refactor the function above to use the `match` statement
    instead of if, else if, and else.
     
    Define a `factorial` function that calculates the
    factorial of a number. The factorial is the product
    of multiplying a number by every incremental
    number leading up to it, starting from 1.
     
    Examples:
    The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
    factorial(5) should return 120.
     
    The factorial of 4 is 4 * 3 * 2 * 1 = 24
    factorial(4) should return 24.
     
    Implement two solutions/functions for the problem.
    The first solution should not use recursion.
    The second solution should use recursion.
    */



fn main() {
    println!("Hello, world!");
    color_to_number("red");
    
    let color = "green";
    match color {
        "red" => println!("This is {color} -> 1"),
        "blue" => println!("This is {color} -> 2"),
        "green" => println!("This is {color} -> 3"),
        _ => println!("This is Unknown -> 0")
    }

    //FaCtorial using loop
    let mut fact = 5;
    let mut result = 1;
    while fact >= 1 {
        result *= fact;
        fact -= 1;
    }
    println!("{}", result);


    // factorial(4);
    println!("The factorial is {:?}", factorial(5))
}


//1
fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        return 1
    }
    else if color == "blue" {
        return 2
    }
    else if color == "green" {
        return 3
    }
    else {
        return 0
    }
}


//RECURSION: factorial
fn factorial (number: i32) -> i32 {
    if number == 1 {
        return 1;
    }
    number * factorial(number - 1)
}
