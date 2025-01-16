## The Result Enum:


### `Result` Enum
- The `Result` enum has two variants:
  - `Ok(T)` – Represents a success with associated data of type `T`.
  - `Err(E)` – Represents a failure with associated error data of type `E`.
  
  The `T` and `E` types are generic, allowing flexibility in what data the `Ok` and `Err` variants hold. `T` represents the type of the success data, while `E` represents the type of the error data.

### Differences from `Option`
- The `Option` enum is used for representing presence (`Some`) or absence (`None`), while `Result` is for representing success (`Ok`) or error (`Err`).
- If something goes wrong in Rust, you would typically return a `Result` to indicate the error and handle it explicitly, unlike other languages that rely on exceptions.

### Handling `Result`
- When declaring a `Result` type, Rust cannot automatically infer both generic types. For example, if we declare a `Result` like `let ok = Result::Ok(5);`, Rust can infer the type of `T` (in this case `i32`), but not the type of `E`. You need to explicitly specify the type for both generics like `Result<i32, &str>` to make it compile.

### Debugging and Displaying
- The `Result` enum implements the `Debug` trait, allowing you to print both the `Ok` and `Err` variants using the `{:?}` formatter. However, it does **not** implement the `Display` trait by default.
  
### Usage Example
- If you declare a `Result` like:
  ```rust
  let ok: Result<i32, &str> = Result::Ok(5);
  let disaster: Result<i32, &str> = Result::Err("Something went wrong");
  ```
  You must provide both types (e.g., `i32` for `T` and `&str` for `E`) explicitly for the `Result` to compile.

The `Result` enum is central in Rust for error handling and helps ensure that both success and failure are handled in a controlled manner.
