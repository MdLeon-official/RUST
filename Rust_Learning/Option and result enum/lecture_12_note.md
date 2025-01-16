## Result Methods


### Methods for Handling `Result` Enums

1. **`unwrap()`**: 
   - This method is used to extract the associated data from the `Ok` variant.
   - If the `Result` is an `Err`, calling `unwrap()` will cause a **runtime panic**.
   - Example:
     ```rust
     let result = divide(10.0, 2.0);
     println!("Result: {}", result.unwrap()); // Will print 5
     ```

2. **`expect()`**:
   - This method works like `unwrap()`, but it allows you to provide a **custom error message**.
   - If the `Result` is an `Err`, the `expect()` method will trigger a panic with the provided message.
   - Example:
     ```rust
     let result = divide(10.0, 0.0);
     println!("Result: {}", result.expect("Unable to parse calculation")); // Will panic with custom message
     ```

3. **`unwrap_or()`**:
   - This method allows you to provide a **fallback value** in case of an `Err`. It returns the fallback value instead of panicking.
   - The type of the fallback value must match the type of the `Ok` variant.
   - Example:
     ```rust
     let result = divide(10.0, 0.0);
     println!("Result: {}", result.unwrap_or(0.0)); // Will print 0.0
     ```

4. **`is_ok()`**:
   - This method checks if the `Result` is an `Ok` variant and returns a `bool`.
   - Example:
     ```rust
     let result = divide(10.0, 2.0);
     println!("{}", result.is_ok()); // Prints true if the result is Ok
     ```

5. **`is_err()`**:
   - This method checks if the `Result` is an `Err` variant and returns a `bool`.
   - Example:
     ```rust
     let result = divide(10.0, 0.0);
     println!("{}", result.is_err()); // Prints true if the result is Err
     ```

### Example Usage

```rust
fn main() {
    let result = divide(10.0, 2.0); // Valid division
    println!("Result using unwrap: {}", result.unwrap()); // Prints 5.0

    let result_with_error = divide(10.0, 0.0); // Division by zero
    // Will panic with a custom message
    println!("Result with error: {}", result_with_error.expect("Unable to parse calculation"));

    // Using unwrap_or to provide a fallback value
    println!("Fallback result: {}", result_with_error.unwrap_or(0.0)); // Prints 0.0

    // Checking if the result is Ok or Err
    println!("Is result Ok? {}", result.is_ok()); // Prints true
    println!("Is result with error Err? {}", result_with_error.is_err()); // Prints true
}
```

### Key Takeaways
- `unwrap()` and `expect()` can be used to extract values from the `Result`, but they panic if the result is `Err`. `expect()` offers a custom error message.
- `unwrap_or()` allows specifying a fallback value when an error occurs.
- `is_ok()` and `is_err()` provide a simple way to check the variant type of the `Result`.
- These methods provide flexibility in how you handle the success (`Ok`) or failure (`Err`) cases, giving you several ways to deal with errors gracefully without always relying on `match`.
