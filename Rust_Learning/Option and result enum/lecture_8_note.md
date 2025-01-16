## Building option from scratch:

---

### **Building the `MyOption` Enum**

1. **Enum Declaration**:
   - The `MyOption` enum has two variants:
     - `Some(i32)`: Holds a 32-bit integer.
     - `None`: Holds no associated data.
   
   ```rust
   #[derive(Debug, Copy, Clone)]
   enum MyOption {
       Some(i32),
       None,
   }
   ```

   - `#[derive(Debug, Copy, Clone)]` is used to automatically implement the `Debug`, `Copy`, and `Clone` traits.
     - `Debug` enables printing the enum using `println!`.
     - `Copy` allows for the enum's values to be copied (because `i32` implements `Copy`).
     - `Clone` ensures the enum can be cloned if needed.

---

### **Implementing the `unwrap` Method**

The `unwrap` method behaves similarly to the standard `Option` enum. It attempts to extract the value in the `Some` variant and panics if it's `None`.

1. **Method Definition**:
   ```rust
   impl MyOption {
       fn unwrap(self) -> i32 {
           match self {
               MyOption::Some(value) => value,
               MyOption::None => panic!("Uh oh"),
           }
       }
   }
   ```

   - The method takes ownership of `self` (since `Copy` is derived, ownership is not lost).
   - It uses a `match` statement to handle both the `Some` and `None` variants:
     - `Some(value)`: Returns the value inside `Some`.
     - `None`: Triggers a panic with a custom error message.

2. **Example Usage**:
   ```rust
   let some_option = MyOption::Some(100);
   println!("{}", some_option.unwrap()); // Output: 100
   
   let none_option = MyOption::None;
   println!("{}", none_option.unwrap()); // Panic: Uh oh
   ```

---

### **Implementing the `unwrap_or` Method**

The `unwrap_or` method provides a fallback value if the enum is `None`, unlike `unwrap` which panics.

1. **Method Definition**:
   ```rust
   impl MyOption {
       fn unwrap_or(self, fallback_value: i32) -> i32 {
           match self {
               MyOption::Some(value) => value,
               MyOption::None => fallback_value,
           }
       }
   }
   ```

   - Similar to `unwrap`, this method takes ownership of `self` and returns an `i32`.
   - In the `None` case, instead of panicking, it returns the provided fallback value.

2. **Example Usage**:
   ```rust
   let some_option = MyOption::Some(100);
   println!("{}", some_option.unwrap_or(13)); // Output: 100
   
   let none_option = MyOption::None;
   println!("{}", none_option.unwrap_or(13)); // Output: 13
   ```

---

### **Key Takeaways**

- **Custom Enum**: We created a custom `MyOption` enum, similar to Rust's built-in `Option`, with two variants: `Some` (holding a value) and `None` (holding no value).
- **Methods**:
  - `unwrap`: Returns the value in `Some` or panics in `None`.
  - `unwrap_or`: Returns the value in `Some` or a provided fallback value in `None`, preventing panics.
- **Copy and Clone**: The `Copy` trait is derived so that we can copy the enum instances without losing ownership.
