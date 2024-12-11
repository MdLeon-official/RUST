fn main() {
    //A function is a sequence of steps to executed in order
    open_store("Barishal"); //assigning value to a parameter
    bake_store(5, "pineapple");
    money();
    open_store("Dhaka");
    bake_store(15, "Pepperoni");

    //RETURN value is the output of a function
    println!("The square is: {}", square_return(5));
    println!("The square is: {}", short_squrare_return(6));

    //THE UNIT AS A RETURN TYPE:    a unit is an empty tuple, a tuple without values;
    let _result = unit();

    //  BLOCKS IN FUNCTION
    let mul = 4;
    let calculation = {
        let value = 5;
        value * mul
    };
    println!("Blocks in function: Mul -> {}", calculation);
}
fn open_store(location: &str) {
    //declaring a parameter
    println!("Opening my pizza store in {location}")
}
fn bake_store(number: i32, topping: &str) {
    println!("Baking {number} {topping} a pizza");
}
fn money() {
    println!("So much money $$$$$");
}

fn square_return(number: i32) -> i32 {
    return number * number;
}
//a shortcut for return: if you dont use return keyword rust will automatically
// return the last line and in this case dont use semicolon
fn short_squrare_return(number: i32) -> i32 {
    number * number
}

//UNIT
fn unit() {}
