# self Parameter as Immutable and Mutable References to Struct Instance


### The Four Ways to Define `self`:

1. **Immutable `self` (Ownership)**:
   - By default, when you pass `self` in a method, it takes ownership of the struct, which means the method can do anything with the struct but cannot return ownership.
   - Example:
     ```rust
     fn display_song_info(self) {
         // method that takes ownership of the struct
     }
     ```

2. **Mutable `self` (Ownership)**:
   - This is similar to the previous case, but the struct can be modified within the method because `self` is mutable.
   - Example:
     ```rust
     fn double_length(mut self) {
         // method that takes mutable ownership of the struct
     }
     ```

3. **Immutable Reference (`&self`)**:
   - Instead of taking ownership, you can take an immutable reference to the struct (`&self`), which means you can access the struct's data without modifying it.
   - Example:
     ```rust
     fn display_song_info(&self) {
         // method that takes an immutable reference to the struct
     }
     ```

4. **Mutable Reference (`&mut self`)**:
   - This allows the method to modify the struct without transferring ownership. The method gets a mutable reference to the struct.
   - Example:
     ```rust
     fn double_length(&mut self) {
         // method that takes a mutable reference to the struct
     }
     ```

### Refactoring and Improvements:

1. **Refactoring `display_song_info`**:
   - Originally, `display_song_info` took ownership of the struct. By refactoring it to accept an immutable reference (`&self`), you avoid losing ownership of the struct, allowing you to call other methods on the same instance.
   - The final refactored version:
     ```rust
     fn display_song_info(&self) {
         // No ownership is taken, can call other methods
     }
     ```

2. **Refactoring `double_length`**:
   - The `double_length` method originally took ownership of the struct mutably (`mut self`). Refactoring it to accept a mutable reference (`&mut self`) ensures you can modify the struct without taking ownership.
   - The final refactored version:
     ```rust
     fn double_length(&mut self) {
         // Modify the struct without taking ownership
     }
     ```

### Key Takeaways:
- **Immutable references (`&self`)**: The method can read from the struct without taking ownership, allowing subsequent method calls on the same instance.
- **Mutable references (`&mut self`)**: The method can modify the struct's state without taking ownership, ensuring that the struct can still be accessed after the method finishes.
- **Ownership**: When you pass `self` (without `&` or `mut`), you transfer ownership, which means you can't use the struct after that unless it’s returned.

### Best Practices:
- **Use references (`&self` and `&mut self`)**: These are preferred because they prevent ownership transfer, allowing multiple methods to operate on the same struct without issues.
- **Use `self` and `mut self` only when you need to take ownership**: This is less common and typically used when ownership is essential (like when you want to transfer ownership or mutate the struct in a method).

