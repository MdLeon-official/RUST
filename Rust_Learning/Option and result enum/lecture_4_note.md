## The match keyword with option enum


### **Why Not `unwrap` and `expect`?**
- **Risky**: They fail at runtime if the `Option` is `None`.
- **Optimistic**: Assume a `Some` variant, which may not always be the case.

---

### **`match` Keyword: The Safer Alternative**
1. **Advantages**:
   - Requires handling **all possible scenarios** (`Some` and `None` variants).
   - Prevents runtime errors by enforcing exhaustive matching.

2. **Example**:

   ```rust
   let bass: Option<&String> = Some(&String::from("Bass"));
   let invalid_instrument: Option<&String> = None;

   match bass {
       Some(instrument) => println!("Playing the {}", instrument),
       None => println!("Singing with my voice"),
   }

   match invalid_instrument {
       Some(instrument) => println!("Playing the {}", instrument),
       None => println!("Singing with my voice"),
   }
   ```

3. **Output**:
   - For `bass`: `Playing the Bass`
   - For `invalid_instrument`: `Singing with my voice`

---

### **Refactoring with a Function**
- To avoid **repeated logic**, extract the `match` block into a function.

```rust
fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Some(instrument) => println!("Playing the {}", instrument),
        None => println!("Singing with my voice"),
    }
}

let bass: Option<&String> = Some(&String::from("Bass"));
let invalid_instrument: Option<&String> = None;

play(bass);
play(invalid_instrument);
```

---

### **Key Notes on Ownership and Copy**
- **`Option` Implements the `Copy` Trait**:
  - Passing an `Option` to a function **does not transfer ownership**; it creates a **copy**.
  - This allows you to use the original `Option` after the function call.

- **Debugging and Inspecting `Option`**:
  - `Option` supports the `Debug` trait, so you can easily inspect its state:
    ```rust
    println!("{:?}", bass); // Outputs: Some("Bass")
    ```

- **Standard Library Insight**:
  - `Option` is lightweight, making copies efficient.
  - You can view its implementation in the Rust source code, where traits like `Copy` and `Debug` are derived.
