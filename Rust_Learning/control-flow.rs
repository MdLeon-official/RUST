fn main() {
    //CONTROL FLOW refers to how a program will execute
    let season: &str = "winter";
    if season == "winter" {
        println!("Its the best")
    } else if season == "summer" {
        println!("No comment")
    } else if season == "spring" {
        println!("Idk what to say")
    } else {
        println!("Blank")
    };

    //Assigning result of if statement to variable
    even_or_odd(5);

    //THE MATCH STATEMENT
    //the match statement works like a switch case
    let evalution: bool = true;
    match evalution {
        //A Pattern or Arm is one possible option to compare the match value against
        true => {
            println!("The value is true.")
        }
        false => {
            println!("The value is false.")
        }
    };
    /* here im using a max statement to check for whatever evaluation is if its true get a value of 1
    and false get a value of 2 and assign that value to value variable */
    let value = match evalution {
        true => 1,
        false => 2,
    };
    println!("Evalution value {}", value);

    //UNDERSCORE IN A MATCH ARM
    //To refactor means to restructure or improve existing code without altering its design
    match season {
        "summer" => println!("Matched summer"),
        "winter" => println!("Matched Winter"),
        "spring" => println!("Matched spring"),
        //always will return the same type
        _ => println!("IDK too many season"),
        //"_" this is a wildcard pattern or a catch-all pattern
    }

    //THE MATCH STATEMENT WITH MULTIPLE VALUES AND CONDITIONALS
    let num = 7;
    match num {
        //*use pipe (|) for multiple value
        // 2 | 4 | 6 | 8 => println!("{} is even", num),
        // 1 | 3 | 7 => println!("{} is odd", num),
        // _ => println!("Unknown for now"),
        value if value % 2 == 0 => println!("{value} is an even number."),
        // This line matches any value where value % 2 == 0 is true (even numbers)
        // and prints that value
        x if x % 2 != 0 => println!("{x} is an odd number"),
        _ => unreachable!(), // This line should never execute since we handle all cases above
    }

    //LOOP - ITERATION
    //loop keyword will always print stopthis
    let mut ll = 21;
    // loop {
    //     if ll <= 0 {
    //         println!("Stop this");
    //         break; //to exit the loop
    //     }
    //     if ll % 2 == 0 {
    //         println!("{ll} seconds even number, skipping 3 seconds.");
    //         ll -= 3;
    //         continue;
    //         //continue keyword forces a loop to move to the next iteration
    //     }
    //     println!("{ll} seconds to blastoff.");
    //     ll -= 1;
    // }

    //WHILE LOOP
    while ll > 0 {
        if ll % 2 == 0 {
            println!("{ll} seconds even number, skipping 3 seconds.");
            ll -= 3;
            continue;
        }
        println!("{ll} seconds to blastoff.");
        ll -= 1;
    }

    //RECURSION
    //recursion is when a function calls itself
    //a BASE CASE is a condition that stops the recursion
    countdown(5);
}

//Assigning result of if statement to variable
fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };

    println!("{} is {}", number, result);
}

//RECURSION
//countdown(5)
// 5 seconds to blastoff
//  countdown(4)
//  4 seconds to blastoff
//    countdown(3)
fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("BOOOOOOOOOOOOM");
    } else {
        println!("Start countdown from {seconds}");
        countdown(seconds - 1);
    }
}
