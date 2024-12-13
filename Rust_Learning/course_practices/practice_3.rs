fn main() {
        /*
    Declare an i32 variable assigned to 1337.
    Use the underscore character to add a visual
    separator between the numbers.
     
    Cast the i32 to an i16 integer and assign the result
    to a separate variable.
     
    Declare a floating-point value of your choosing.
    Print out the number with 3 digits of precision.
     
    Declare a 'with_milk' variable set to a Boolean.
    Declare a 'with_sugar` variable set to a Boolean.
     
    Declare a 'is_my_type_of_coffee` variable. It should
    be set to true if the coffee has both milk and sugar.
     
    Declare an `is_acceptable_coffee` variable. It should
    be set to true if the coffee has either milk or
    sugar.
     
    Declare an array with four i8 integers of your choosing
    Print out the array in its Debug representation.
     
    Declare a tuple consisting of the integer, float,
    a Boolean, and the array that you previously declared.
    Print out the tuple in its Debug representation.
    */

    let pln = "C:\\home\\leon\\RUST_code";
    println!("This is a file path {}",pln);
    let pln = r"C:\home\leon\RUST_code";
    println!("This is a file path {}",pln);

    let one:i32 = 1_3_3_7;
    let _two:i16 = one as i16;
    let pi: f64 = 3.14159;
    println!("PI = {:.3}",pi);

    let with_milk = true;
    let with_sugar = true;
    if with_milk && with_sugar{
        let is_my_type_of_coffee = true;
        println!("Yes this is my type of coffee {}", is_my_type_of_coffee);
    }
    if with_milk || with_sugar{
        let is_acceptable_coffee = true;
        println!("This is accepted {}", is_acceptable_coffee);
    }

    let arr:[i8; 4] = [4,5,6,7];
    for i in 0..4{
        println!("{}",arr[i]);
    }

    let tup = (2334, 23.3423, true);
    let (tup_int, tup_float, tup_bool) = tup;
    println!("Integer in this tuple {tup_int} Float in this tuple {tup_float} Boolean in this tuple {tup_bool}");
    println!("{tup:#?}")
}
