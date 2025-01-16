## Top-level option variants:

### **Key Points about the Rust Prelude**
1. **Automatic Availability**:
   - Types like `Option`, `Result`, `String`, `Vec`, and more are part of the prelude.
   - Common traits like `Clone`, `Copy`, and `Debug` are also included.
   - This saves you from manually importing them into every program.

2. **The `Option` Enum**:
   - The `Option` enum is part of the prelude, so you can use `Option` directly in your code without importing it.
   - The two variants of `Option`—`Some` and `None`—are also automatically available at the top level.

3. **Simplified Syntax**:
   - You can use `Some` and `None` directly without prefixing them with `Option::`.
   - For example:
     ```rust
     Some(42) // Simplified
     Option::Some(42) // Fully qualified (less common)
     ```
   - This simplification is purely syntactic. Both approaches are functionally identical.

4. **When the Full `Option` Enum is Needed**:
   - You must use `Option` when referring to the enum itself, such as when defining the return type of a function:
     ```rust
     fn get_value() -> Option<i32> {
         Some(10)
     }
     ```
   - However, inside the function body, you can use `Some` and `None` directly:
     ```rust
     if condition {
         Some(42)
     } else {
         None
     }
     ```

5. **Manual Imports for Other Constructs**:
   - While the prelude covers many essentials, some features must be explicitly imported. For example:
     - `HashMap` from `std::collections`
     - `File` from `std::fs`
   - These are not included in the prelude because they are less commonly used than constructs like `Option`.

---

### **Example Code Simplifying the Previous Lesson**
Here’s the refactored version of the `is_item_in_stock` function using the shortcut syntax:

```rust
fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Some(true) // Simplified syntax for 'Option::Some'
    } else if item_is_in_system {
        Some(false)
    } else {
        None // Simplified syntax for 'Option::None'
    }
}

fn main() {
    let availability = is_item_in_stock(true, true);
    match availability {
        Some(true) => println!("Yes, the item is available."),
        Some(false) => println!("No, the item is not in stock."),
        None => println!("Your item doesn't exist in our system."),
    }
}
```

---

### **Why the Prelude Exists**
The Rust prelude is designed to:
- Reduce boilerplate code by making common constructs readily available.
- Ensure that you can focus on writing logic without repeatedly importing the same types or traits.

By understanding the prelude and how it works, you can write cleaner, more efficient Rust programs.
