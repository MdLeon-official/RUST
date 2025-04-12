# Methods with Multiple Parameters


### Defining Methods with Parameters:
- The first parameter in every method **must always** be `self`. This is required, and it can be one of the four options (i.e., `self`, `mut self`, `&self`, or `&mut self`).
- After `self`, you can define any number of additional parameters, just like regular functions.
- When calling a method, you don't need to provide the `self` parameter; Rust will automatically pass it in for you.
- You only need to pass arguments for the other parameters.

### Example: `is_longer_than` Method
- The `is_longer_than` method is designed to compare two `TaylorSwiftSong` structs based on their `duration_secs`.
- The method takes a reference to `self` as the first parameter, and the second parameter is an immutable reference to another `TaylorSwiftSong` struct (for comparison).
  
```rust
fn is_longer_than(&self, other: &Self) -> bool {
    self.duration_secs > other.duration_secs
}
```

- In the main function, you create two `TaylorSwiftSong` structs (`blank_space` and `all_too_well`) and compare them using the `is_longer_than` method. 

### Key Takeaways:
- We can define methods with additional parameters, and those must come after the `self` parameter.
- The first parameter (`self`) is implicitly handled by Rust when we invoke the method.
- When invoking a method, you only need to provide arguments for the custom parameters (e.g., `other`), not `self`.
