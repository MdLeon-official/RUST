## Real example of Result enum

### The `parse` Method
- The `parse` method is used to attempt converting a string into another data type. It returns a `Result` enum, where:
  - **`Ok(T)`**: Contains the successfully parsed value (e.g., an `i32` if the conversion succeeds).
  - **`Err(E)`**: Contains an error (e.g., `ParseIntError`) if the conversion fails.
  
### Example 1: Successful Parsing
```rust
let text = "50";
let text_as_number: Result<i32, _> = text.parse(); // using turbofish to specify type
println!("{:?}", text_as_number);
```
- In this case, the string "50" is successfully parsed into an `i32`, and the result is the `Ok` variant containing the integer `50`.
  
### Example 2: Unsuccessful Parsing
```rust
let text = "Alabama";
let text_as_number: Result<i32, _> = text.parse();
println!("{:?}", text_as_number);
```
- If the string doesn't contain a valid number (e.g., "Alabama"), the `parse` method will return an `Err` variant, containing a `ParseIntError`.

### Key Takeaways
- The `Result` enum allows you to consistently handle success and failure. The return value is always a `Result`, but the content will either be the success data in the `Ok` variant or the error data in the `Err` variant.
- Rust's `parse` method is a great example of how functions return a `Result`, and it forces you to handle both success and failure scenarios explicitly. 

In short, the `Result` enum is used in Rust to provide a consistent way to deal with operations that may succeed or fail, encouraging you to handle errors properly.
