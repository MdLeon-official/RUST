## The unwrap_or method


The `unwrap_or` method in Rust provides a safe and concise way to extract a value from an `Option` enum while offering a fallback in case the value is `None`.

---

### **Key Points about `unwrap_or`**

1. **Purpose**:
   - Extracts the value inside the `Some` variant.
   - Provides a fallback value in case the `Option` is `None`.
   - Avoids runtime panics by handling the `None` case gracefully.

2. **Method Signature**:
   ```rust
   fn unwrap_or(self, default: T) -> T
   ```
   - `self`: The `Option` instance (e.g., `Some(value)` or `None`).
   - `default`: The fallback value to return if the `Option` is `None`.
   - Returns the value of the `Some` variant or the fallback `default` value.

3. **Usage Examples**:
   ```rust
   let present_value = Some(13); // Option<i32>
   let missing_value: Option<i32> = None;

   println!("{}", present_value.unwrap_or(0)); // Output: 13
   println!("{}", missing_value.unwrap_or(0)); // Output: 0
   ```

4. **Type Safety**:
   - The type of the fallback value must match the type stored in the `Option`.
   - If there is a mismatch, the Rust compiler will produce an error.
   - Example:
     ```rust
     let option_value: Option<i32> = Some(42);
     println!("{}", option_value.unwrap_or(true)); // Compiler error
     ```

5. **Advantages**:
   - Avoids panics that might occur with `unwrap` on a `None` value.
   - Simplifies error handling by providing a backup value.

---

### **Code Example with Explanation**

Here's the code demonstrated in the lesson:

```rust
fn main() {
    // Example 1: Present value in 'Some' variant
    let present_value = Some(13); // Option<i32>
    println!("Present value: {}", present_value.unwrap_or(0)); // Output: 13

    // Example 2: Missing value with 'None' variant
    let missing_value: Option<i32> = None;
    println!("Missing value with fallback: {}", missing_value.unwrap_or(0)); // Output: 0

    // Example 3: Changing the fallback value
    println!("Missing value with custom fallback: {}", missing_value.unwrap_or(100)); // Output: 100

    // Example 4: Type mismatch handling
    // Uncommenting the following line would cause a compiler error:
    // println!("{}", missing_value.unwrap_or(true)); // Mismatched types: expected `i32`, found `bool`

    // Correcting the type mismatch
    let bool_value: Option<bool> = Some(true);
    println!("Boolean value: {}", bool_value.unwrap_or(false)); // Output: true
}
```

---

### **Takeaways**
- The `unwrap_or` method provides a more robust alternative to `unwrap`, as it eliminates the risk of runtime panics by requiring a fallback value.
- It enforces type safety, ensuring that fallback values align with the type stored in the `Option`.
- Use `unwrap_or` for safe and clear value extraction when dealing with optional data.
