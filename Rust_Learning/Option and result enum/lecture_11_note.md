## Returning a Result enum from a function:


### Custom `divide` Function
The `divide` function takes two `f64` parameters: a `numerator` and a `denominator`. It returns a `Result`:
- **`Ok(f64)`**: If the division is successful, it returns the result of the division as an `f64`.
- **`Err(String)`**: If the denominator is zero (an invalid operation), it returns an error with a message in the form of a `String`.

### Function Definition
```rust
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(numerator / denominator)
}
```
- If `denominator` is zero, the function returns an `Err` with an appropriate error message.
- Otherwise, it performs the division and returns the result wrapped in the `Ok` variant.

### Handling the `Result` in `main`
In the `main` function, you call `divide` and handle the returned `Result` using the `match` keyword:

```rust
let result = divide(10.0, 2.0); // Valid division

match result {
    Ok(calculation) => println!("Result: {}", calculation),
    Err(message) => println!("Error: {}", message),
}
```
This allows you to gracefully handle both success and failure:
- **`Ok`**: Outputs the result of the division.
- **`Err`**: Outputs the error message.

### Example with Division by Zero
```rust
let result = divide(10.0, 0.0); // Division by zero

match result {
    Ok(calculation) => println!("Result: {}", calculation),
    Err(message) => println!("Error: {}", message),
}
```
- When dividing by zero, the function returns an `Err` with the message `"Cannot divide by zero"`, which is printed as an error.

### Key Takeaways
1. **`Result` enum**: The `Result` enum is used to represent either success (`Ok`) or failure (`Err`) in Rust functions.
2. **Graceful error handling**: Instead of panicking, you can return meaningful error messages using `Err`, allowing the caller to handle it appropriately.
3. **`match` for handling results**: The `match` statement is commonly used to handle the two possible outcomes (`Ok` and `Err`) from functions returning `Result`.
